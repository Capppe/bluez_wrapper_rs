extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bt_wrapper::{adapter::adapter1::Adapter1, Properties};
    use dbus::{
        arg::{RefArg, Variant},
        Path,
    };

    #[test]
    fn test_method_get_discovery_filters() {
        let iface = Adapter1::new().unwrap();

        let res = iface.get_discovery_filters().unwrap();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_remove_device() {
        let iface = Adapter1::new().unwrap();

        let res = iface.remove_device(Path::from("test")).unwrap();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_set_discovery_filter() {
        let iface = Adapter1::new().unwrap();

        let map: HashMap<String, Variant<Box<dyn RefArg>>> = HashMap::new();

        let res = iface.set_discovery_filter(map).unwrap();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_start_discovery() {
        let iface = Adapter1::new().unwrap();

        let iface_c = iface.clone();
        let _ = iface_c.start_discovery();

        assert!(1 == 1)
    }

    #[test]
    fn test_method_stop_discovery() {
        let iface = Adapter1::new().unwrap();

        let iface_c = iface.clone();
        let _ = iface_c.stop_discovery();

        assert!(1 == 1)
    }

    // Properties tests
    #[test]
    fn test_props_get_roles() {
        let iface = Adapter1::new().unwrap();

        let roles = iface.get_property::<Vec<String>>("Roles");

        println!("Exp feat: {:?}", roles);
    }

    #[test]
    fn test_props_get_discoverable() {
        let iface = Adapter1::new().unwrap();

        let discoverable = iface.get_property::<bool>("Discoverable");

        println!("Exp feat: {:?}", discoverable);
    }

    #[test]
    fn test_props_get_address() {
        let iface = Adapter1::new().unwrap();

        let address = iface.get_property::<String>("Address");

        println!("Exp feat: {:?}", address);
    }

    #[test]
    fn test_props_get_all() {
        let iface = Adapter1::new().unwrap();

        let props = iface.get_all_properties::<Vec<String>>().unwrap();

        println!("All props: {:?}", props);
    }
}
