use dbus::{blocking::SyncConnection, Path};

use crate::{DBusItem, DBusProxy, Methods};

pub struct BatteryProviderManager1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for BatteryProviderManager1 {
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

impl<'a> DBusProxy<'a> for BatteryProviderManager1 {}

impl Methods for BatteryProviderManager1 {}

impl BatteryProviderManager1 {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.bluez.BatteryProviderManager1".to_string(),
            object_path: "/org/bluez/hci0".to_string(),
            connection: SyncConnection::new_system()?,
        })
    }

    // Methods
    pub fn register_battery_provider(&self, provider: Path) -> Result<(), String> {
        self.call_method_no_return("RegisterBatteryProvider", (provider,))
    }

    pub fn unregister_battery_provider(&self, provider: Path) -> Result<(), String> {
        self.call_method_no_return("UnregisterBatteryProvider", (provider,))
    }
}
