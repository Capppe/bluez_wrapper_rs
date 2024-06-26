use std::{collections::HashMap, time::Duration};

use crate::device::Device;
use crate::errors::BluetoothError;
use crate::utils::get_device_properties;
use dbus::blocking::stdintf::org_freedesktop_dbus::ObjectManager;
// use dbus::ffidisp::stdintf::org_freedesktop_dbus::ObjectManager;
use dbus::{arg::RefArg, blocking::Connection};

pub struct Bluetooth {
    connection: Connection,
    adapter_path: String,
}

impl Bluetooth {
    pub fn new() -> Result<Self, BluetoothError> {
        // init connection
        let connection = Connection::new_system()?;
        let adapter_path = String::from("/org/bluez/hci0"); //Default path
        Ok(Self {
            connection,
            adapter_path,
        })
    }

    pub fn discover_devices(&self) -> Result<Vec<Device>, BluetoothError> {
        // device discovery logic
        let proxy = self.connection.with_proxy(
            "org.bluez",
            &self.adapter_path,
            Duration::from_millis(5000),
        );

        proxy.method_call("org.bluez.Adapter1", "StartDiscovery", ())?;

        std::thread::sleep(Duration::from_secs(5));

        let mut devices: Vec<Device> = Vec::new();

        let manager_proxy =
            self.connection
                .with_proxy("org.bluez", "/", Duration::from_millis(5000));

        let managed_objects: HashMap<
            dbus::Path<'_>,
            HashMap<String, HashMap<String, dbus::arg::Variant<Box<dyn RefArg>>>>,
        > = manager_proxy.get_managed_objects()?;

        for (_path, interfaces) in managed_objects {
            if let Some(properties) = interfaces.get("org.bluez.Device1") {
                devices.push(get_device_properties(properties));
            }
        }

        proxy.method_call("org.bluez.Adapter1", "StopDiscovery", ())?;

        Ok(devices)
    }

    pub fn connect(&self, device: &str) -> Result<(), BluetoothError> {
        // device connection logic
        Ok(())
    }
}
