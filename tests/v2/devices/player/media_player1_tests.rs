extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use bt_wrapper::{devices::player::media_player1::MediaPlayer1, Properties};
    use dbus::Path;

    const DEV_ADDR: &str = "64:A2:F9:F2:FA:EC";

    #[test]
    fn test_method_fast_forward() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res = iface.fast_forward();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_hold() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res = iface.hold(1);

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_next() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res = iface.next();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_pause() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res = iface.pause();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_play() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res = iface.play();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_press() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res = iface.press(1);

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_previous() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res = iface.previous();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_release() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res = iface.release();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_rewind() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res = iface.rewind();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_method_stop() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res = iface.stop();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_props_get_browsable() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res: bool = iface.get_property("Browsable").unwrap();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }

    #[test]
    fn test_props_get_device() {
        let iface = MediaPlayer1::new(DEV_ADDR).unwrap();

        let res: Path = iface.get_property("Device").unwrap();

        println!("Res: {:?}", res);

        assert!(1 == 1)
    }
}
