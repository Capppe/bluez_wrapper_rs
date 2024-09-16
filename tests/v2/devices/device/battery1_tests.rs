extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    const DEV_ADDR: &str = "64:A2:F9:F2:FA:EC";
    use bt_wrapper::{devices::device::battery1::Battery1, Properties};

    #[test]
    fn test_props_get_percentage() {
        let iface = Battery1::new(DEV_ADDR).unwrap();

        let res = iface.get_property::<u8>("Percentage");

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_props_get_source() {
        let iface = Battery1::new(DEV_ADDR).unwrap();

        let res = iface.get_property::<String>("Source");

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }
}
