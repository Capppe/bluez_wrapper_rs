extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use bt_wrapper::bluetooth::Bluetooth;

    #[test]
    fn test_discovery() {
        let bt = Bluetooth::new().expect("Failed to initialize Bluetooth");
        println!("Bluetooth initialized");

        let devices = bt.discover_devices().expect("Failed to discover devices");
        println!("Devices found: {:?}", devices);

        assert!(!devices.is_empty());
    }
}
