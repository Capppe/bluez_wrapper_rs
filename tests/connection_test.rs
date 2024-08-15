extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bt_wrapper::bluetooth::Bluetooth;

    #[tokio::test]
    async fn test_connection() {
        run_test().await;
    }

    async fn run_test() {
        let bt = Bluetooth::new()
            .await
            .expect("Failed to initialize Bluetooth");

        let connect_result = bt
            .connect(String::from("64:A2:F9:F2:FA:EC"), Duration::from_secs(10))
            .await
            .expect("Failed to connect from device");

        assert!(connect_result == ());

        let disconnect_result = bt
            .disconnect(String::from("64:A2:F9:F2:FA:EC"), Duration::from_secs(10))
            .await
            .expect("Failed to disconnect from device");

        assert!(disconnect_result == ());
    }
}
