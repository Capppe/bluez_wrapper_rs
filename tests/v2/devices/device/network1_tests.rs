extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    const DEV_ADDR: &str = "64:A2:F9:F2:FA:EC";
    use bt_wrapper::{devices::device::network1::Network1, Properties};

    #[test]
    fn test_method_connect() {
        let iface = Network1::new(DEV_ADDR).unwrap();

        let res = iface.connect("1234-test-tset-4321").unwrap();

        println!("Res: {}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_disconnect() {
        let iface = Network1::new(DEV_ADDR).unwrap();

        let res = iface.disconnect();

        assert!(res == Ok(()))
    }

    #[test]
    fn test_props_get_connected() {
        let iface = Network1::new(DEV_ADDR).unwrap();

        let res: bool = iface.get_property("Connected").unwrap();

        println!("Res: {}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_props_get_interface() {
        let iface = Network1::new(DEV_ADDR).unwrap();

        let res: String = iface.get_property("Interface").unwrap();

        println!("Res: {}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_props_get_uuid() {
        let iface = Network1::new(DEV_ADDR).unwrap();

        let res: String = iface.get_property("UUID").unwrap();

        println!("Res: {}", res);

        assert!(1 == 1)
    }
}
