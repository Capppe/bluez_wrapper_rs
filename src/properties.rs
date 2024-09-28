use std::{collections::HashMap, future::Future};

use dbus::{
    arg::{Append, Arg, RefArg, Variant},
    blocking::SyncConnection,
    Message,
};
use tokio::sync::mpsc::Sender;

use crate::{dbus_utils::parse_propmap, DBusItem, DBusProxy, Methods, Signals};

pub struct Properties {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for Properties {
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

impl<'a> DBusProxy<'a> for Properties {}

impl Signals for Properties {}

impl Methods for Properties {}

impl Properties {
    pub fn new(path: &str) -> Result<Self, String> {
        Ok(Self {
            interface: "org.freedesktop.DBus.Properties".to_string(),
            object_path: path.to_string(),
            connection: SyncConnection::new_system()
                .map_err(|e| format!("Failed to acquire dbus-connection: {}", e))?,
        })
    }

    // Methods
    pub fn get<T>(&self, interface_name: &str, property_name: &str) -> Result<T, String>
    where
        T: Arg + for<'z> dbus::arg::Get<'z>,
    {
        self.call_method("Get", (interface_name, property_name))
    }

    pub fn get_all<T>(&self, interface_name: &str) -> Result<T, String>
    where
        T: Arg + for<'z> dbus::arg::Get<'z>,
    {
        self.call_method("GetAll", (interface_name,))
    }

    pub fn set<T>(&self, interface_name: &str, property_name: &str, value: T) -> Result<(), String>
    where
        T: Append + dbus::arg::Arg,
    {
        self.call_method_no_return("Set", (interface_name, property_name, value))
    }

    // Signals
    pub fn properties_changed<'a>(
        &'a self,
        sender: Sender<HashMap<String, String>>,
        interface: Option<&'a str>,
    ) -> impl Future + 'a {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Ok((path, props, _arr)) = message.read_all::<(
                    String,
                    HashMap<String, Variant<Box<dyn RefArg>>>,
                    Vec<String>,
                )>() {
                    let mut msg = parse_propmap(&props);
                    msg.insert("Sender".to_owned(), path);

                    let _ = sender.send(msg).await;
                }
            }
        });

        self.start_listener(
            tx,
            interface.unwrap_or(self.get_interface()),
            "PropertiesChanged",
        )
    }
}
