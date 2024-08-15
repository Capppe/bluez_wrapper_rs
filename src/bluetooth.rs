use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use crate::bt_status::Status;
use crate::device::Device;
use crate::errors::BluetoothError;
use crate::utils::get_device_properties;
use crate::utils::get_path_from_address;
use dbus::arg::PropMap;
use dbus::nonblock::MethodReply;
use dbus::nonblock::Proxy;
use dbus::nonblock::SyncConnection;
use dbus_tokio::connection;
use regex::Regex;
use tokio::time::timeout;

#[derive(Clone)]
pub struct Bluetooth {
    pub connection: Arc<SyncConnection>,
    pub adapter_path: String,
    pub current_device: Option<Device>,
}

//TODO add traits

// trait Metods {
//     async fn test() -> Result<(), Box<dyn Error>>;
// }

impl Bluetooth {
    pub async fn new() -> Result<Self, BluetoothError> {
        let (resource, connection) = connection::new_system_sync()?;

        let _handle = tokio::spawn(async {
            let err = resource.await;
            panic!("Lost connection to DBus: {}", err);
        });

        let adapter_path = String::from("/org/bluez/hci0"); //Default path
        Ok(Self {
            connection,
            adapter_path,
            current_device: None,
        })
    }

    pub async fn get_status(&self) -> Result<Status, BluetoothError> {
        let adapter = self.adapter_path.clone();
        let conn = self.connection.clone();

        let proxy = Proxy::new("org.bluez", adapter, Duration::from_millis(5000), conn);
        let (mut result,): (PropMap,) = proxy
            .method_call(
                "org.freedesktop.DBus.Properties",
                "GetAll",
                ("org.bluez.Adapter1",),
            )
            .await
            .unwrap();

        let devices = self.get_known_devices().await;
        let mut connected_devices = 0;

        if devices.is_ok() {
            for device in devices.unwrap() {
                if let Some(c) = device.connected {
                    if c == true {
                        connected_devices += 1;
                    }
                }
            }
        }

        result.insert(
            "ConnectedDevices".to_string(),
            dbus::arg::Variant(Box::new(connected_devices)),
        );

        Ok(Status::new(result))
    }

    pub async fn start_discovery(
        &self,
        discovery_duration: Duration,
    ) -> Result<(), BluetoothError> {
        let adapter = self.adapter_path.clone();
        let conn = self.connection.clone();

        let discovery_future: tokio::task::JoinHandle<Result<(), _>> = tokio::spawn(async move {
            let proxy = Proxy::new("org.bluez", adapter, Duration::from_millis(5000), conn);

            proxy
                .method_call("org.bluez.Adapter1", "StartDiscovery", ())
                .await
                .map_err(|_| BluetoothError::StartDiscoveryFailed)?;

            tokio::time::sleep(discovery_duration).await;

            Ok::<(), BluetoothError>(())
        });

        match timeout(
            discovery_duration + Duration::from_secs(1),
            discovery_future,
        )
        .await
        {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(BluetoothError::Unknown(e.to_string())),
            Err(_) => Err(BluetoothError::Timeout),
        }
    }

    pub async fn stop_discovery(&self) -> Result<(), BluetoothError> {
        let conn = self.connection.clone();
        let adapter = self.adapter_path.clone();

        let proxy = Proxy::new("org.bluez", adapter, Duration::from_millis(5000), conn);

        proxy
            .method_call("org.bluez.Adapter1", "StopDiscovery", ())
            .await
            .map_err(|_| BluetoothError::StopDiscoveryFailed)?;

        Ok(())
    }

    pub async fn connect(
        &self,
        address: String,
        timeout_duration: Duration,
    ) -> Result<(), BluetoothError> {
        let conn = self.connection.clone();
        let adapter = get_path_from_address(&address, &self.adapter_path);

        let proxy = Proxy::new("org.bluez", adapter, timeout_duration, conn);

        let result = proxy
            .method_call("org.bluez.Device1", "Connect", ())
            .await
            .map_err(|e| BluetoothError::ConnectionFailed(e.to_string()));

        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(BluetoothError::Unknown(e.to_string())),
        }
    }

    pub async fn disconnect(
        &self,
        address: String,
        timeout_duration: Duration,
    ) -> Result<(), BluetoothError> {
        let conn = self.connection.clone();
        let adapter = get_path_from_address(&address, &self.adapter_path);

        let proxy = Proxy::new("org.bluez", adapter, timeout_duration, conn);

        let result = proxy
            .method_call("org.bluez.Device1", "Disconnect", ())
            .await;

        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(BluetoothError::Unknown(e.to_string())),
        }
    }

    pub async fn get_known_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        let conn = self.connection.clone();
        let proxy = Proxy::new("org.bluez", "/", Duration::from_secs(5), conn);
        let mut devices = Vec::new();

        let result: (HashMap<dbus::Path, HashMap<String, dbus::arg::PropMap>>,) = proxy
            .method_call(
                "org.freedesktop.DBus.ObjectManager",
                "GetManagedObjects",
                (),
            )
            .await?;

        let device_path_regex = Regex::new(r"/org/bluez/hci0/.*").unwrap();

        for (key, value) in result.0 {
            if device_path_regex.is_match(&key) {
                for (k, v) in value {
                    if k == "org.bluez.Device1" {
                        let device = get_device_properties(&v);
                        devices.push(device);
                    }
                }
            }
        }

        Ok(devices)
    }
}
