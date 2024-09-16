extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bt_wrapper::{root::object_manager::ObjectManager, DBusItem};
    use dbus::{arg::PropMap, Path};

    #[tokio::test]
    async fn test_method_get_managed_objects() {
        let iface = ObjectManager::new().unwrap();

        let res = iface.get_managed_objects().unwrap();

        println!("Res: {:?}", res);

        assert!(!res.is_empty())
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
