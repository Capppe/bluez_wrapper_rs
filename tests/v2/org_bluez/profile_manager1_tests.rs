extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bt_wrapper::org_bluez::profile_manager1::ProfileManager1;
    use dbus::{
        arg::{RefArg, Variant},
        Path,
    };

    // Fails, find out how to use
    #[tokio::test]
    async fn test_method_register_profile() {
        let iface = ProfileManager1::new().unwrap();

        let map: HashMap<String, Variant<Box<dyn RefArg>>> = HashMap::new();

        let res = iface
            .register_profile(Path::from("/test"), "test-1234-tset-4321".to_string(), map)
            .unwrap();

        assert!(res == ())
    }

    // Fails, find out how to use
    #[tokio::test]
    async fn test_method_unregister_profile() {
        let iface = ProfileManager1::new().unwrap();

        let res = iface.unregister_profile(Path::from("/test")).unwrap();

        assert!(res == ())
    }
}
