use dbus::{blocking::SyncConnection, Path};

use crate::{DBusItem, DBusProxy, Methods};

pub struct AgentManager1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for AgentManager1 {
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

impl<'a> DBusProxy<'a> for AgentManager1 {}

impl Methods for AgentManager1 {}

impl AgentManager1 {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.bluez.AgentManager1".to_string(),
            object_path: "/org/bluez".to_string(),
            connection: SyncConnection::new_system()?,
        })
    }

    // Methods
    pub fn register_agent(&self, agent: Path, capability: String) -> Result<(), String> {
        self.call_method_no_return("RegisterAgent", (agent, capability))
    }

    pub fn request_default_agent(&self, agent: Path) -> Result<(), String> {
        self.call_method_no_return("RequestDefaultAgent", (agent,))
    }

    pub fn unregister_agent(&self, agent: Path) -> Result<(), String> {
        self.call_method_no_return("UnregisterAgent", (agent,))
    }
}
