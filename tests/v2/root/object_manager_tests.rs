extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use std::{
        any::{Any, TypeId},
        collections::HashMap,
    };

    use bt_wrapper::{dbus_utils, root::object_manager::ObjectManager, DBusItem};
    use dbus::{
        arg::{PropMap, RefArg, Variant},
        Path,
    };

    #[tokio::test]
    async fn test_method_get_managed_objects() {
        let iface = ObjectManager::new().unwrap();

        let res = iface.get_managed_objects().unwrap();

        println!("Res: {:?}", res);

        assert!(!res.is_empty())
    }

    #[test]
    fn test_method_parse_variant() {
        let str: Variant<Box<dyn RefArg>> = Variant(Box::new("Hej".to_string()));
        let int: Variant<Box<dyn RefArg>> = Variant(Box::new(12 as i32));
        let float: Variant<Box<dyn RefArg>> = Variant(Box::new(12.0 as u32));
        let bool: Variant<Box<dyn RefArg>> = Variant(Box::new(true));

        let p_str = dbus_utils::parse_variant::<String>(&str).expect("Failed");
        let p_int = dbus_utils::parse_variant::<i32>(&int).expect("Failed");
        let p_float = dbus_utils::parse_variant::<u32>(&float).expect("Failed");
        let p_bool = dbus_utils::parse_variant::<bool>(&bool).expect("Failed");

        assert!(p_str.type_id() == TypeId::of::<String>());
        assert!(p_int.type_id() == TypeId::of::<i32>());
        assert!(p_float.type_id() == TypeId::of::<u32>());
        assert!(p_bool.type_id() == TypeId::of::<bool>())
    }

    #[tokio::test]
    async fn test_signal_interfaces_added() {
        let iface = ObjectManager::new().unwrap();
        let (tx, mut rx) =
            tokio::sync::mpsc::channel::<(Path<'static>, HashMap<String, PropMap>)>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message-0: {}", message.0);
                println!("Message-1: {:?}", message.1);
            }
        });

        let _res = iface
            .interfaces_added(tx, Some(iface.get_interface()))
            .await;

        assert!(1 == 2)
    }

    #[tokio::test]
    async fn test_signal_interfaces_removed() {
        let iface = ObjectManager::new().unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel::<(Path<'static>, Vec<String>)>(100);

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                println!("Message-0: {}", message.0);
                println!("Message-1: {:?}", message.1);
            }
        });

        let _res = iface
            .interfaces_removed(tx, Some(iface.get_interface()))
            .await;

        assert!(1 == 2)
    }
}
