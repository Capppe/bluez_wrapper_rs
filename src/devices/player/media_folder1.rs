use std::collections::HashMap;

use dbus::{arg::PropMap, blocking::SyncConnection, Path};

use crate::{
    utils::{get_path_from_address, validate_address},
    DBusItem, DBusProxy, Methods, Properties,
};

pub struct MediaFolder1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for MediaFolder1 {
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

impl<'a> DBusProxy<'a> for MediaFolder1 {}

impl Methods for MediaFolder1 {}

impl Properties for MediaFolder1 {}

impl MediaFolder1 {
    pub fn new(dev_address: &str) -> Result<Self, String> {
        if !validate_address(dev_address) {
            return Err(format!("Invalid address: {}", dev_address));
        };

        let dev_address = get_path_from_address(dev_address, "/org/bluez/hci0");
        Ok(Self {
            interface: "org.bluez.MediaFolder1".to_string(),
            object_path: format!("{}", dev_address),
            connection: SyncConnection::new_system()
                .map_err(|e| format!("Failed to acquire dbus-connection: {}", e))?,
        })
    }

    // Methods
    pub fn change_folder(&self, folder: Path) -> Result<(), String> {
        self.call_method_no_return("ChangeFolder", (folder,))
    }

    pub fn list_items(&self, filter: PropMap) -> Result<HashMap<Path, PropMap>, String> {
        self.call_method("ListItems", (filter,))
    }

    pub fn search(&self, string: String, filter: PropMap) -> Result<Path, String> {
        self.call_method("Search", (string, filter))
    }
}
