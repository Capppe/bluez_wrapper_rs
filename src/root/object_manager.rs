use std::collections::HashMap;

use dbus::{arg::PropMap, blocking::SyncConnection, Message, Path};
use tokio::sync::mpsc::Sender;

use crate::{DBusItem, DBusProxy, Methods, Signals};

pub struct ObjectManager {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for ObjectManager {
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

impl<'a> DBusProxy<'a> for ObjectManager {}

impl Methods for ObjectManager {}

impl Signals for ObjectManager {}

impl ObjectManager {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self {
            interface: "org.freedesktop.DBus.ObjectManager".to_string(),
            object_path: "/".to_string(),
            connection: SyncConnection::new_system()?,
        })
    }

    // Methods
    // TODO: Remove propmap usage
    pub fn get_managed_objects(&self) -> Result<HashMap<Path, HashMap<String, PropMap>>, String> {
        self.call_method("GetManagedObjects", ())
    }

    // Signals
    // TODO: Remove propmap usage
    pub async fn interfaces_added(
        &self,
        sender: Sender<(Path<'static>, HashMap<String, PropMap>)>,
        interface: Option<&str>,
    ) -> Result<(), String> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Ok((path, props)) =
                    message.read_all::<(Path<'static>, HashMap<String, PropMap>)>()
                {
                    let _ = sender.send((path, props)).await;
                }
            }
        });

        let _ = self
            .start_listener(
                tx,
                interface.unwrap_or(self.get_interface()),
                "InterfacesAdded",
            )
            .await;

        Ok(())
    }

    pub async fn interfaces_removed(
        &self,
        sender: Sender<(Path<'static>, Vec<String>)>,
        interface: Option<&str>,
    ) -> Result<(), String> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Ok((path, props)) = message.read_all() {
                    let _ = sender.send((path, props)).await;
                }
            }
        });

        let _ = self
            .start_listener(
                tx,
                interface.unwrap_or(self.get_interface()),
                "InterfacesRemoved",
            )
            .await;

        Ok(())
    }
}
