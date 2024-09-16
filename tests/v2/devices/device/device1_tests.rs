extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    const DEV_ADDR: &str = "64:A2:F9:F2:FA:EC";
    use bt_wrapper::{devices::device::device1::Device1, Properties};

    #[test]
    fn test_method_cancel_pairing() {
        let iface = Device1::new(DEV_ADDR).unwrap();

        let res = iface.cancel_pairing().unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_connect() {
        let iface = Device1::new(DEV_ADDR).unwrap();

        let res = iface.connect().unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_connect_profile() {
        let iface = Device1::new(DEV_ADDR).unwrap();

        let res = iface
            .connect_profile("1234-test-tset-4321".to_string())
            .unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_disconnect() {
        let iface = Device1::new(DEV_ADDR).unwrap();

        let res = iface.disconnect().unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_disconnect_profile() {
        let iface = Device1::new(DEV_ADDR).unwrap();

        let res = iface
            .disconnect_profile("1234-test-tset-4321".to_string())
            .unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_method_pair() {
        let iface = Device1::new(DEV_ADDR).unwrap();

        let res = iface.pair().unwrap();

        assert!(res == ())
    }

    #[test]
    fn test_props_get_uuids() {
        let iface = Device1::new(DEV_ADDR).unwrap();

        let res = iface.get_property::<Vec<String>>("UUIDs");

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_props_get_connected() {
        let iface = Device1::new(DEV_ADDR).unwrap();

        let res = iface.get_property::<bool>("Connected");

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_props_get_address() {
        let iface = Device1::new(DEV_ADDR).unwrap();

        let res = iface.get_property::<String>("Address");

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_props_get_all() {
        let iface = Device1::new(DEV_ADDR).unwrap();

        let res = iface.get_all_properties::<Vec<String>>();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }
}
