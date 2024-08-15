use crate::bluetooth::Bluetooth;
use crate::device::{Device, Json};
use crate::errors::BluetoothError;
use crate::utils::{get_address_from_path, get_path_from_address};
use async_channel::Sender;
use dbus::blocking::Proxy;
use dbus::message::MatchRule;
use dbus::nonblock::MsgMatch;
use dbus::Message;
use std::collections::HashMap;
use std::future;
use std::time::Duration;

#[derive(Clone)]
pub struct Listener {
    bluetooth: Bluetooth,
}

trait Callbacks {
    fn device_found_callback(message: Message, sender: Sender<String>);
    fn device_removed_callback(message: Message, sender: Sender<String>);
    // fn device_connected_callback(message: Message, sender: Sender<String>);
    fn device_properties_changed_callback(message: Message, sender: Sender<String>);
}

impl Listener {
    /// Creates a new Listener struct
    pub async fn new(bt: Bluetooth) -> Result<Self, BluetoothError> {
        Ok(Self { bluetooth: bt })
    }

    pub async fn start_device_added_listener(
        &self,
        sender: Sender<String>,
    ) -> Result<MsgMatch, BluetoothError> {
        let signal = self
            .init_listener(
                "InterfacesAdded".to_string(),
                None,
                None,
                sender,
                Self::device_found_callback,
            )
            .await?;

        Ok(signal)
    }

    pub async fn start_device_removed_listener(
        &self,
        sender: Sender<String>,
    ) -> Result<MsgMatch, BluetoothError> {
        let signal = self
            .init_listener(
                "InterfacesRemoved".to_string(),
                None,
                None,
                sender,
                Self::device_removed_callback,
            )
            .await?;

        Ok(signal)
    }

    async fn init_listener(
        &self,
        signal_name: String,
        path: Option<String>,
        iface: Option<String>,
        sender: Sender<String>,
        cb: impl Fn(Message, Sender<String>) + Send + Sync + 'static,
    ) -> Result<MsgMatch, BluetoothError> {
        let conn = &self.bluetooth.connection;
        let proxy = Proxy::new(
            "org.bluez",
            path.unwrap_or("/".to_string()),
            Duration::from_secs(5),
            conn.clone(),
        );

        dbg!("Starting listener for {}", &signal_name);

        let rule = MatchRule::new_signal(
            iface.unwrap_or("org.freedesktop.DBus.ObjectManager".to_string()),
            signal_name,
        );

        let signal = proxy.connection.add_match(rule).await?.cb(
            move |message, (_path, _interface): (dbus::Path<'static>, dbus::arg::PropMap)| {
                let sender = sender.clone();
                cb(message, sender);
                true
            },
        );

        future::pending::<()>().await;

        self.bluetooth
            .connection
            .remove_match(signal.token())
            .await?;

        Ok(signal)
    }

    pub async fn start_device_listener(
        &self,
        address: String,
        sender: Sender<String>,
    ) -> Result<MsgMatch, BluetoothError> {
        let conn = self.bluetooth.connection.clone();
        let adapter_path = self.bluetooth.adapter_path.clone();
        let proxy_path = get_path_from_address(&address, &adapter_path);
        let proxy = Proxy::new("org.bluez", proxy_path, Duration::from_secs(5), conn);

        let rule = MatchRule::new_signal("org.freedesktop.DBus.Properties", "PropertiesChanged");

        let signal = proxy.connection.add_match(rule).await?.cb(
            move |message, (_path, _interface): (String, dbus::arg::PropMap)| {
                let sender = sender.clone();
                Self::device_properties_changed_callback(message, sender);
                true
            },
        );

        dbg!("Device listener started for device: {}", address);

        future::pending::<()>().await;

        self.bluetooth
            .connection
            .remove_match(signal.token())
            .await?;

        Ok(signal)
    }
}

impl Callbacks for Listener {
    fn device_found_callback(message: Message, sender: Sender<String>) {
        tokio::spawn(async move {
            let (_path, data): (dbus::Path<'static>, HashMap<String, dbus::arg::PropMap>) =
                match message.read2() {
                    Ok(res) => res,
                    Err(e) => {
                        dbg!("Error reading message: {:?}", e);
                        return;
                    }
                };

            for (key, value) in data.into_iter() {
                match key.as_str() {
                    "org.bluez.Device1" => {
                        if let Ok(device) = Device::new_from_dbus_message(value) {
                            sender
                                .send(device.to_json().unwrap())
                                .await
                                .expect("Channel is closed");
                        }
                    }
                    &_ => {}
                }
            }
        });
    }

    fn device_removed_callback(message: Message, sender: Sender<String>) {
        tokio::spawn(async move {
            let (path, _data): (dbus::Path<'static>, HashMap<String, dbus::arg::PropMap>) =
                match message.read2() {
                    Ok(res) => res,
                    Err(e) => {
                        dbg!("Error reading message: {:?}", e);
                        return;
                    }
                };

            if let Ok(p) = get_address_from_path(path) {
                sender.send(p).await.expect("Channel is closed");
            }
        });
    }

    fn device_properties_changed_callback(message: Message, sender: Sender<String>) {
        tokio::spawn(async move {
            let (path, _data): (String, dbus::arg::PropMap) = match message.read2() {
                Ok(res) => res,
                Err(e) => {
                    dbg!("Error reading message: {:?}", e);
                    return;
                }
            };

            for (key, value) in _data.into_iter() {
                if path == "org.bluez.Device1" {
                    if key == "Connected" {
                        let c = dbus::arg::cast::<bool>(&value.0)
                            .ok_or(BluetoothError::InvalidDevice)
                            .copied();

                        if let Ok(connected) = c {
                            sender
                                .send(connected.to_string())
                                .await
                                .expect("Channel is closed");
                        }
                    };
                }
            }
        });
    }
}
