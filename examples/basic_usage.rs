extern crate bt_wrapper;

use std::time::Duration;

use bt_wrapper::bluetooth::Bluetooth;

#[tokio::main]
async fn main() {
    // Acquire connection to Bluez DBus API
    let bt = Bluetooth::new()
        .await
        .expect("Failed to initialize Bluetooth");

    scanning_example(&bt).await;

    connect_example(&bt).await;
}

async fn scanning_example(bt: &Bluetooth) {
    // Start the discovery for 5 seconds
    // 0 seconds = infinite
    let _result = bt
        .start_discovery(Duration::from_secs(5))
        .await
        .expect("Failed to discover devices");
}

async fn connect_example(bt: &Bluetooth) {
    // Connect to a device with its mac_address, and assign timeout duration
    let _result = bt.connect(String::from("your:address:here"), Duration::from_secs(10));
}
