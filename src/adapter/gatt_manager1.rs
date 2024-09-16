use dbus::{arg::PropMap, blocking::SyncConnection, Path};

use crate::{DBusItem, DBusProxy, Methods};

pub struct GattManager1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for GattManager1 {
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

impl<'a> DBusProxy<'a> for GattManager1 {}

impl Methods for GattManager1 {}

impl GattManager1 {
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

    pub fn unregister_application(&self, application: Path) -> Result<(), String> {
        self.call_method_no_return("UnregisterApplication", (application,))
    }
}
