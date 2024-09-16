extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use bt_wrapper::{devices::player::media_folder1::MediaFolder1, Properties};
    use dbus::{arg::PropMap, Path};

    const DEV_ADDR: &str = "64:A2:F9:F2:FA:EC";

    // TODO: find out what this does
    #[test]
    fn test_method_change_folder() {
        let iface = MediaFolder1::new(DEV_ADDR).unwrap();

        let res = iface.change_folder(Path::from("/test"));

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    // TODO: and this...
    fn test_method_list_items() {
        let iface = MediaFolder1::new(DEV_ADDR).unwrap();

        let res = iface.list_items(PropMap::new());

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    // TODO: and this...
    fn test_method_search() {
        let iface = MediaFolder1::new(DEV_ADDR).unwrap();

        let res = iface.search("test".to_string(), PropMap::new());

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_props_get_name() {
        let iface = MediaFolder1::new(DEV_ADDR).unwrap();

        let res: String = iface.get_property("Name").unwrap();

        println!("Res: {}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_props_get_number_of_items() {
        let iface = MediaFolder1::new(DEV_ADDR).unwrap();

        let res: u32 = iface.get_property("NumberOfItems").unwrap();

        println!("Res: {}", res);

        assert!(1 == 1)
    }
}
