extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    const DEV_ADDR: &str = "64:A2:F9:F2:FA:EC";
    use bt_wrapper::{devices::device::media_control1::MediaControl1, Properties};
    use dbus::Path;

    #[test]
    fn test_method_fast_forward() {
        let iface = MediaControl1::new(DEV_ADDR).unwrap();

        let res = iface.fast_forward();

        assert!(res == Ok(()))
    }

    #[test]
    fn test_method_next() {
        let iface = MediaControl1::new(DEV_ADDR).unwrap();

        let res = iface.next();

        assert!(res == Ok(()))
    }

    #[test]
    fn test_method_pause() {
        let iface = MediaControl1::new(DEV_ADDR).unwrap();

        let res = iface.pause();

        assert!(res == Ok(()))
    }

    #[test]
    fn test_method_play() {
        let iface = MediaControl1::new(DEV_ADDR).unwrap();

        let res = iface.play();

        assert!(res == Ok(()))
    }

    #[test]
    fn test_method_previous() {
        let iface = MediaControl1::new(DEV_ADDR).unwrap();

        let res = iface.previous();

        assert!(res == Ok(()))
    }

    #[test]
    fn test_method_rewind() {
        let iface = MediaControl1::new(DEV_ADDR).unwrap();

        let res = iface.rewind();

        assert!(res == Ok(()))
    }

    #[test]
    fn test_method_stop() {
        let iface = MediaControl1::new(DEV_ADDR).unwrap();

        let res = iface.stop();

        assert!(res == Ok(()))
    }

    #[test]
    fn test_method_volume_down() {
        let iface = MediaControl1::new(DEV_ADDR).unwrap();

        let res = iface.volume_down();

        assert!(res == Ok(()))
    }

    #[test]
    fn test_method_volume_up() {
        let iface = MediaControl1::new(DEV_ADDR).unwrap();

        let res = iface.volume_up();

        assert!(res == Ok(()))
    }

    #[test]
    fn test_props_get_connected() {
        let iface = MediaControl1::new(DEV_ADDR).unwrap();

        let res: bool = iface.get_property("Connected").unwrap();

        println!("Res: {}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_props_get_player() {
        let iface = MediaControl1::new(DEV_ADDR).unwrap();

        let res: Path = iface.get_property("Player").unwrap();

        println!("Res: {}", res);

        assert!(1 == 1)
    }
}
