use dbus::blocking::SyncConnection;

use crate::{utils::get_path_from_address, DBusItem, DBusProxy, Methods, Properties};

pub struct Network1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for Network1 {
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

impl<'a> DBusProxy<'a> for Network1 {}

impl Methods for Network1 {}

impl Properties for Network1 {}

impl Network1 {
    pub fn new(dev_address: &str) -> Result<Self, dbus::Error> {
        let dev_address = get_path_from_address(dev_address, "/org/bluez/hci0");
        Ok(Self {
            interface: "org.bluez.Network1".to_string(),
            object_path: format!("{}", dev_address),
            connection: SyncConnection::new_system()?,
        })
    }

    // Methods
    pub fn connect(&self, uuid: &str) -> Result<String, String> {
        self.call_method("Connect", (uuid,))
    }

    pub fn disconnect(&self) -> Result<(), String> {
        self.call_method_no_return("Disconnect", ())
    }
}
