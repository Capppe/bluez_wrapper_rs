use dbus::{arg::PropMap, blocking::SyncConnection, Path};

use crate::{DBusItem, DBusProxy, Methods, Properties};

pub struct Media1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for Media1 {
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

impl<'a> DBusProxy<'a> for Media1 {}

impl Methods for Media1 {}

impl Properties for Media1 {}

impl Media1 {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.bluez.Adapter1".to_string(),
            object_path: "/org/bluez/hci0".to_string(),
            connection: SyncConnection::new_system()?,
        })
    }

    // Methods
    pub fn register_application(&self, application: Path, options: PropMap) -> Result<(), String> {
        // TODO: remove propmap usage
        self.call_method_no_return("RegisterApplication", (application, options))
    }

    pub fn register_endpoint(&self, endpoint: Path, properties: PropMap) -> Result<(), String> {
        // TODO: remove propmap usage
        self.call_method_no_return("RegisterEndpoint", (endpoint, properties))
    }

    pub fn register_player(&self, player: Path, properties: PropMap) -> Result<(), String> {
        // TODO: remove propmap usage
        self.call_method_no_return("RegisterPlayer", (player, properties))
    }

    pub fn unregister_application(&self, application: Path) -> Result<(), String> {
        self.call_method_no_return("UnregisterApplication", (application,))
    }

    pub fn unregister_endpoint(&self, endpoint: Path) -> Result<(), String> {
        self.call_method_no_return("UnregisterEndpoint", (endpoint,))
    }

    pub fn unregister_player(&self, player: Path) -> Result<(), String> {
        self.call_method_no_return("UnregisterPlayer", (player,))
    }
}
