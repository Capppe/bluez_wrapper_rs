use dbus::{arg::PropMap, blocking::SyncConnection, Path};

use crate::{DBusItem, DBusProxy, Methods, Properties};

pub struct LEAdvertisingManager1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for LEAdvertisingManager1 {
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

impl<'a> DBusProxy<'a> for LEAdvertisingManager1 {}

impl Methods for LEAdvertisingManager1 {}

impl Properties for LEAdvertisingManager1 {}

impl LEAdvertisingManager1 {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.bluez.Adapter1".to_string(),
            object_path: "/org/bluez/hci0".to_string(),
            connection: SyncConnection::new_system()?,
            //proxy: None,
        })
    }

    // Methods
    pub fn register_advertisement(
        &self,
        advertisement: Path,
        options: PropMap,
    ) -> Result<(), String> {
        // TODO: remove propmap usage
        self.call_method_no_return("RegisterAdvertisement", (advertisement, options))
    }

    pub fn unregister_advertisement(&self, service: Path) -> Result<(), String> {
        self.call_method_no_return("UnregisterAdvertisement", (service,))
    }
}
