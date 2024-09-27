use dbus::blocking::SyncConnection;

use crate::{utils::validate_path, DBusItem, DBusProxy, Methods};

pub struct Introspectable {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for Introspectable {
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

impl<'a> DBusProxy<'a> for Introspectable {}

impl Methods for Introspectable {}

impl Introspectable {
    pub fn new(path: &str) -> Result<Self, String> {
        if !validate_path(path) {
            return Err(format!("Invalid path: {}", path));
        }
        Ok(Self {
            interface: "org.freedesktop.DBus.Properties".to_string(),
            object_path: path.to_string(),
            connection: SyncConnection::new_system()
                .map_err(|e| format!("Failed to acquire dbus-connection: {}", e))?,
        })
    }

    // Methods
    pub fn introspect(&self) -> Result<String, String> {
        self.call_method("Introspect", ())
    }
}
