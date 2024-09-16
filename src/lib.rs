use std::{collections::HashMap, time::Duration};

use dbus::{
    arg::{PropMap, Variant},
    blocking::SyncConnection,
    message::MatchRule,
    Message,
};
use dbus_utils::parse_propmap;
use tokio::sync::mpsc::Sender;

pub mod introspectable;
pub mod utils;

// V2
pub mod adapter;
pub mod dbus_utils;
pub mod devices;
pub mod org_bluez;
pub mod properties;
pub mod root;

pub trait DBusItem {
    fn get_interface(&self) -> &str;
    fn get_object_path(&self) -> &str;
    fn get_connection(&self) -> &SyncConnection;
}

pub trait DBusProxy<'a> {
    fn get_new_proxy(
        &'a self,
        dest: Option<&'a str>,
        object_path: Option<&'a str>,
    ) -> Result<dbus::blocking::Proxy<&SyncConnection>, String>
    where
        Self: DBusItem,
    {
        Ok(dbus_utils::create_proxy(
            dest,
            object_path.unwrap_or(&self.get_object_path()),
            Duration::from_secs(5),
            &self.get_connection(),
        )?)
    }
}

pub trait Signals {
    fn start_listener(
        &self,
        sender: Sender<Message>,
        interface: &str,
        signal: &str,
    ) -> impl std::future::Future<Output = Result<(), String>> + Send {
        async move {
            let (resource, connection) = dbus_tokio::connection::new_system_sync()
                .map_err(|e| format!("Failed to get DBus connection(async): {}", e))?;

            tokio::spawn(async {
                let err = resource.await;
                panic!("Lost connection to DBus: {}", err);
            });

            let rule = MatchRule::new_signal(interface.to_owned(), signal.to_owned());

            use futures_util::stream::StreamExt;
            let (incoming_signal, stream) = connection
                .add_match(rule)
                .await
                .map_err(|e| format!("Failed to add signal match rule: {}", e))?
                .stream();

            let stream = stream.for_each(|(msg, _): (Message, ())| {
                let sender = sender.clone();
                tokio::spawn(async move {
                    let _ = sender.send(msg).await;
                });
                async {}
            });

            futures_util::join!(stream);

            connection
                .remove_match(incoming_signal.token())
                .await
                .map_err(|e| format!("Failed to remove signal match rule: {}", e))?;

            Ok(())
        }
    }
}

pub trait Methods {
    fn call_method<T, A>(&self, method: &str, args: A) -> Result<T, String>
    where
        T: for<'z> dbus::arg::Get<'z> + dbus::arg::Arg,
        A: dbus::arg::AppendAll,
        Self: for<'a> DBusProxy<'a> + DBusItem,
    {
        let proxy = self.get_new_proxy(None, None)?;

        let (value,): (T,) = proxy
            .method_call(self.get_interface(), method, args)
            .map_err(|e| format!("Failed to call method {}, cause: {}", method, e))?;

        Ok(value)
    }

    fn call_method_no_return<A>(&self, method: &str, args: A) -> Result<(), String>
    where
        A: dbus::arg::AppendAll,
        Self: for<'a> DBusProxy<'a> + DBusItem,
    {
        let proxy = self.get_new_proxy(None, None)?;

        proxy
            .method_call(self.get_interface(), method, args)
            .map_err(|e| format!("Failed to call method {}, cause: {}", method, e))?;

        Ok(())
    }
}

pub trait Properties {
    fn get_property<T>(&self, property: &str) -> Result<T, String>
    where
        Self: DBusItem,
        T: dbus::arg::Arg + for<'z> dbus::arg::Get<'z>,
    {
        if let Ok(props) = properties::Properties::new(self.get_object_path()) {
            let prop: Variant<T> = props.get(&self.get_interface(), property)?;
            Ok(prop.0)
        } else {
            Err(format!("Failed to get properties"))
        }
    }

    fn get_all_properties<T>(&self) -> Result<HashMap<String, String>, String>
    where
        Self: DBusItem,
        T: dbus::arg::Arg + for<'z> dbus::arg::Get<'z>,
    {
        if let Ok(props) = properties::Properties::new(self.get_object_path()) {
            let propmap: PropMap = props.get_all(&self.get_interface()).map_err(|e| {
                format!(
                    "Failed to get all properties for {}, cause: {}",
                    self.get_interface(),
                    e
                )
            })?;
            let map = parse_propmap(&propmap);
            Ok(map)
        } else {
            Err(format!("Failed to get all properties"))
        }
    }

    fn set_property<T>(&self, property: &str, value: T) -> Result<(), String>
    where
        Self: DBusItem,
        T: dbus::arg::Arg + for<'z> dbus::arg::Get<'z> + dbus::arg::Append,
    {
        if let Ok(props) = properties::Properties::new(self.get_object_path()) {
            props.set(&self.get_interface(), property, value)
        } else {
            Err(format!("Failed to set properties"))
        }
    }
}
