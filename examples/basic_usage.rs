extern crate bt_wrapper;

use bt_wrapper::bluetooth::Bluetooth;

fn main() {
    let bt = Bluetooth::new().expect("Failed to initialize Bluetooth");

    let devices = bt.discover_devices().expect("Failed to discover devices");
    for device in devices {
        println!("Found device: {:?}", device);
    }

    bt.connect("Device1").expect("Failed to connect to device");
}
