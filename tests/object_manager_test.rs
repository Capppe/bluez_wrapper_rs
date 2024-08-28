extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use bt_wrapper::bluetooth::Bluetooth;

    #[tokio::test]
    async fn test_get_paired_devices() {
        let bt = Bluetooth::new(None)
            .await
            .expect("Failed to initialize Bluetooth");

        let result = bt.get_known_devices().await;

        println!("Result: {:?}", result);
    }
}
