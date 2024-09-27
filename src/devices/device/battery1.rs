use dbus::blocking::SyncConnection;

use crate::{
    utils::{get_path_from_address, validate_address},
    DBusItem, Properties,
};

pub struct Battery1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for Battery1 {
    fn get_interface(&self) -> &str {
        &self.interface
    }

    fn get_object_path(&self) -> &str {
        &self.object_path
    }

    fn get_connection(&self) -> &SyncConnection {
        &self.connection
    }
}

impl Properties for Battery1 {}

impl Battery1 {
    pub fn new(dev_address: &str) -> Result<Self, String> {
        if !validate_address(dev_address) {
            return Err(format!("Invalid address: {}", dev_address));
        }

        let dev_address = get_path_from_address(dev_address, "/org/bluez/hci0");
        Ok(Self {
            interface: "org.bluez.Battery1".to_string(),
            object_path: format!("{}", dev_address),
            connection: SyncConnection::new_system()
                .map_err(|e| format!("Failed to acquire dbus-connection: {}", e))?,
        })
    }
}
