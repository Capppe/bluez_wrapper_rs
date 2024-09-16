use dbus::blocking::SyncConnection;

use crate::{DBusItem, DBusProxy, Methods};

pub struct NetworkServer1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for NetworkServer1 {
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

impl<'a> DBusProxy<'a> for NetworkServer1 {}

impl Methods for NetworkServer1 {}

impl NetworkServer1 {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.bluez.Adapter1".to_string(),
            object_path: "/org/bluez/hci0".to_string(),
            connection: SyncConnection::new_system()?,
        })
    }

    // Methods
    pub fn register(&self, uuid: String, bridge: String) -> Result<(), String> {
        self.call_method_no_return("Register", (uuid, bridge))
    }

    pub fn unregister(&self, uuid: String) -> Result<(), String> {
        self.call_method_no_return("Unregister", (uuid,))
    }
}
