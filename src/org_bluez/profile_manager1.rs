use dbus::{arg::PropMap, blocking::SyncConnection, Path};

use crate::{DBusItem, DBusProxy, Methods};

pub struct ProfileManager1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for ProfileManager1 {
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

impl<'a> DBusProxy<'a> for ProfileManager1 {}

impl Methods for ProfileManager1 {}

impl ProfileManager1 {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.bluez.ProfileManager1".to_string(),
            object_path: "/org/bluez".to_string(),
            connection: SyncConnection::new_system()?,
        })
    }

    // Methods
    pub fn register_profile(
        &self,
        profile: Path,
        uuid: String,
        options: PropMap,
    ) -> Result<(), String> {
        self.call_method_no_return("RegisterProfile", (profile, uuid, options))
    }

    pub fn unregister_profile(&self, profile: Path) -> Result<(), String> {
        self.call_method_no_return("UnregisterProfile", (profile,))
    }
}
