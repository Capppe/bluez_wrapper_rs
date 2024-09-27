use std::sync::Arc;

use dbus::{arg::PropMap, blocking::SyncConnection, Path};

use crate::{DBusItem, DBusProxy, Methods, Properties};

#[derive(Clone)]
pub struct Adapter1 {
    interface: String,
    object_path: String,
    connection: Arc<SyncConnection>,
    //proxy: Option<Proxy<'static, &'static Connection>>, // TODO: lifetime??
}

impl DBusItem for Adapter1 {
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

impl<'a> DBusProxy<'a> for Adapter1 {}

impl Methods for Adapter1 {}

impl Properties for Adapter1 {}

impl Adapter1 {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.bluez.Adapter1".to_string(),
            object_path: "/org/bluez/hci0".to_string(),
            connection: Arc::new(SyncConnection::new_system()?),
            //proxy: None,
        })
    }

    // Methods
    pub fn get_discovery_filters(&self) -> Result<Vec<String>, String> {
        self.call_method("GetDiscoveryFilters", ())
    }

    pub fn remove_device(&self, device: Path) -> Result<(), String> {
        self.call_method_no_return("RemoveDevice", (device,))
    }

    pub fn set_discovery_filter(&self, properties: PropMap) -> Result<(), String> {
        self.call_method_no_return("SetDiscoveryFilters", (properties,))
    }

    pub fn start_discovery(&self) -> Result<(), String> {
        let clone = self.clone();
        std::thread::spawn(move || {
            let _ = clone.call_method_no_return("StartDiscovery", ());
            loop {}
        });
        Ok(())
    }

    pub fn stop_discovery(&self) -> Result<(), String> {
        self.call_method_no_return("StopDiscovery", ())
    }

    // TODO: add properties here?
}
