extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    const DEV_ADDR: &str = "64:A2:F9:F2:FA:EC";
    use bt_wrapper::{
        devices::device::device1::Device1,
        utils::{validate_address, validate_path},
        Properties,
    };
    use dbus::arg::{Append, RefArg};

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

    #[test]
    fn test_validate_address_and_path() {
        let corr_addr = "A2:A5:B4:F8:FA:CB";
        let corr_addr2 = "A2:B5:B9:D8:F9:AB";

        let wrong_addr = "2:B5:B9:D8:F9:AB";
        let wrong_addr2 = "A2:B5:B9:D8:F9:";
        let wrong_addr3 = "A2:B5:B9:(8:F9:AB";
        let wrong_addr4 = "A2:B5:B9::F9:AB";
        let wrong_addr5 = "abc-123";

        assert!(validate_address(corr_addr));
        assert!(validate_address(corr_addr2));

        assert!(!validate_address(wrong_addr));
        assert!(!validate_address(wrong_addr2));
        assert!(!validate_address(wrong_addr3));
        assert!(!validate_address(wrong_addr4));
        assert!(!validate_address(wrong_addr5));

        assert!(validate_path(
            &("/org/bluez/hci0/dev_".to_string() + &corr_addr.replace(":", "_"))
        ));
        assert!(validate_path(
            &("/org/bluez/hci0/dev_".to_string() + &corr_addr2.replace(":", "_"))
        ));
        assert!(!validate_path(
            &("/org/bluez/hci0/dev_".to_string() + &wrong_addr.replace(":", "_"))
        ));
        assert!(!validate_path(
            &("/org/bluez/hci0/dev_".to_string() + &wrong_addr2.replace(":", "_"))
        ));
        assert!(!validate_path(
            &("/org/bluez/hci0/dev_".to_string() + &wrong_addr3.replace(":", "_"))
        ));
        assert!(!validate_path(
            &("/org/bluez/hci0/dev_".to_string() + &wrong_addr4.replace(":", "_"))
        ));
        assert!(!validate_path(
            &("/org/bluez/hci0/dev_".to_string() + &wrong_addr5.replace(":", "_"))
        ));
    }
}
