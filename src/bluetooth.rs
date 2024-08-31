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
use dbus::nonblock::stdintf::org_freedesktop_dbus::Properties;
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

pub trait Power {
    fn power_on(&self) -> impl std::future::Future<Output = Result<(), String>>;
    fn power_off(&self) -> impl std::future::Future<Output = Result<(), String>>;
}

impl Bluetooth {
    pub async fn new(adapter_path: Option<String>) -> Result<Self, BluetoothError> {
        let (resource, connection) = connection::new_system_sync()?;

        tokio::spawn(async {
            let err = resource.await;
            panic!("Lost connection to DBus: {}", err);
        });

        let adapter_path = adapter_path.unwrap_or_else(|| "/org/bluez/hci0".to_string());
        Ok(Self {
            connection,
            adapter_path,
            current_device: None,
        })
    }

    fn create_proxy(
        &self,
        path: Option<String>,
        timeout_duration: Duration,
    ) -> Proxy<&SyncConnection> {
        Proxy::new(
            "org.bluez",
            path.unwrap_or_else(|| self.adapter_path.to_string()),
            timeout_duration,
            self.connection.as_ref(),
        )
    }

    pub async fn get_status(&self) -> Result<Status, BluetoothError> {
        // let adapter = &self.adapter_path;
        // let conn = self.connection.to_owned();

        // let proxy = Proxy::new("org.bluez", adapter, Duration::from_millis(5000), conn);
        let proxy = self.create_proxy(None, Duration::from_secs(5));
        let (mut result,): (PropMap,) = proxy
            .method_call(
                "org.freedesktop.DBus.Properties",
                "GetAll",
                ("org.bluez.Adapter1",),
            )
            .await
            .map_err(|_| BluetoothError::Unknown("Failed to call method GetAll".to_owned()))?;

        let devices = self.get_known_devices().await;
        let mut connected_devices = 0;

        if devices.is_ok() {
            connected_devices = devices.unwrap().iter().map(|d| d.connected).len();
        }

        result.insert(
            "ConnectedDevices".to_string(),
            dbus::arg::Variant(Box::new(connected_devices as i32)),
        );

        Ok(Status::new(result))
    }

    pub async fn start_discovery(
        &self,
        discovery_duration: Duration,
    ) -> Result<(), BluetoothError> {
        let adapter = self.adapter_path.clone();
        let conn = self.connection.clone();

        let discovery_future = tokio::spawn(async move {
            let proxy = Proxy::new("org.bluez", adapter, Duration::from_millis(5000), conn);

            proxy
                .method_call("org.bluez.Adapter1", "StartDiscovery", ())
                .await
                .map_err(|_| BluetoothError::StartDiscoveryFailed)?;

            tokio::time::sleep(discovery_duration).await;
            Ok::<(), BluetoothError>(())
        });

        let _ = timeout(
            discovery_duration + Duration::from_secs(1),
            discovery_future,
        )
        .await
        .map_err(|_| BluetoothError::Timeout)?;

        Ok(())

        // match timeout(
        //     discovery_duration + Duration::from_secs(1),
        //     discovery_future,
        // )
        // .await
        // {
        //     Ok(Ok(_)) => Ok(()),
        //     Ok(Err(e)) => Err(BluetoothError::Unknown(e.to_string())),
        //     Err(_) => Err(BluetoothError::Timeout),
        // }
    }

    pub async fn stop_discovery(&self) -> Result<(), BluetoothError> {
        // let conn = self.connection.clone();
        // let adapter = &self.adapter_path;
        //
        // let proxy = Proxy::new("org.bluez", adapter, Duration::from_millis(5000), conn);
        let proxy = self.create_proxy(None, Duration::from_secs(5));

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
        // let conn = self.connection.clone();
        let adapter = get_path_from_address(&address, &self.adapter_path);

        // let proxy = Proxy::new("org.bluez", adapter, timeout_duration, conn);
        let proxy = self.create_proxy(Some(adapter), timeout_duration);

        proxy
            .method_call("org.bluez.Device1", "Connect", ())
            .await
            .map_err(|e| BluetoothError::ConnectionFailed(e.to_string()))
    }

    pub async fn disconnect(
        &self,
        address: String,
        timeout_duration: Duration,
    ) -> Result<(), BluetoothError> {
        // let conn = self.connection.clone();
        let adapter = get_path_from_address(&address, &self.adapter_path);

        // let proxy = Proxy::new("org.bluez", adapter, timeout_duration, conn);
        let proxy = self.create_proxy(Some(adapter), timeout_duration);

        proxy
            .method_call("org.bluez.Device1", "Disconnect", ())
            .await
            .map_err(|e| BluetoothError::DisconnectFailed(e.to_string()))
    }

    pub async fn get_known_devices(&self) -> Result<Vec<Device>, Box<dyn Error>> {
        // let conn = self.connection.clone();
        // let proxy = Proxy::new("org.bluez", "/", Duration::from_secs(5), conn);
        let proxy = self.create_proxy(Some("/".to_string()), Duration::from_secs(5));
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

impl Power for Bluetooth {
    async fn power_on(&self) -> Result<(), String> {
        let proxy = self.create_proxy(Some("/org/bluez/hci0".to_string()), Duration::from_secs(5));

        proxy
            .set("org.bluez.Adapter1", "Powered", true)
            .await
            .map_err(|e| format!("Failed to power on bluetooth: {}", e))
    }

    async fn power_off(&self) -> Result<(), String> {
        let proxy = self.create_proxy(Some("/org/bluez/hci0".to_string()), Duration::from_secs(5));

        proxy
            .set("org.bluez.Adapter1", "Powered", false)
            .await
            .map_err(|e| format!("Failed to power on bluetooth: {}", e))
    }
}
