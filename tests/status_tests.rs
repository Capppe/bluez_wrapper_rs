extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use bt_wrapper::bluetooth::Bluetooth;

    #[tokio::test]
    async fn test_get_status() {
        let bt = Bluetooth::new(None)
            .await
            .expect("Failed to initialize Bluetooth");

        let result = bt.get_status().await;

        println!("Result: {:?}", result);

        assert!(result.is_ok())
    }
}
