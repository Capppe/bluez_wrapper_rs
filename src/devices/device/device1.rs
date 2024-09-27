use dbus::blocking::SyncConnection;

use crate::{
    utils::{get_path_from_address, validate_address},
    DBusItem, DBusProxy, Methods, Properties,
};

pub struct Device1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for Device1 {
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

impl<'a> DBusProxy<'a> for Device1 {}

impl Methods for Device1 {}

impl Properties for Device1 {}

impl Device1 {
    pub fn new(dev_address: &str) -> Result<Self, String> {
        if !validate_address(dev_address) {
            return Err(format!("Invalid address: {}", dev_address));
        }

        let dev_address = get_path_from_address(dev_address, "/org/bluez/hci0");
        Ok(Self {
            interface: "org.bluez.Device1".to_string(),
            object_path: format!("{}", dev_address),
            connection: SyncConnection::new_system()
                .map_err(|e| format!("Failed to acquire dbus-connection: {}", e))?,
        })
    }

    // Methods
    pub fn cancel_pairing(&self) -> Result<(), String> {
        self.call_method_no_return("CancelPairing", ())
    }

    pub fn connect(&self) -> Result<(), String> {
        self.call_method_no_return("Connect", ())
    }

    pub fn connect_profile(&self, uuid: String) -> Result<(), String> {
        self.call_method_no_return("ConnectProfile", (uuid,))
    }

    pub fn disconnect(&self) -> Result<(), String> {
        self.call_method_no_return("Disconnect", ())
    }

    pub fn disconnect_profile(&self, uuid: String) -> Result<(), String> {
        self.call_method_no_return("DisconnectProfile", (uuid,))
    }

    pub fn pair(&self) -> Result<(), String> {
        self.call_method_no_return("Pair", ())
    }
}
