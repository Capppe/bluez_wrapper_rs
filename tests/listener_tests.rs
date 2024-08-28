extern crate bt_wrapper;

#[cfg(test)]
mod tests {

    use bt_wrapper::bluetooth::Bluetooth;
    use bt_wrapper::bt_listener::Listener;

    #[tokio::test]
    async fn test_listener() {
        let bt = Bluetooth::new(None)
            .await
            .expect("Failed to initialize Bluetooth");

        let bt_listener = Listener::new(bt.clone())
            .await
            .expect("Failed to initialize BluetoothListener");

        start_listener(bt_listener).await;
    }

    async fn start_listener(listener: Listener) {
        let (sender, receiver) = async_channel::bounded(1);

        tokio::spawn(async move {
            let _result = listener.start_device_added_listener(sender).await;
        });

        while let Ok(response) = receiver.recv().await {
            println!("Response: {}", response);
        }
        println!("Listener done!");
    }

    #[tokio::test]
    async fn test_removed_listener() {
        let bt = Bluetooth::new(None)
            .await
            .expect("Failed to initialize Bluetooth");

        let bt_listener = Listener::new(bt.clone())
            .await
            .expect("Failed to initialize BluetoothListener");

        start_removed_listener(bt_listener).await;
    }

    async fn start_removed_listener(listener: Listener) {
        let (sender, receiver) = async_channel::bounded(1);

        tokio::spawn(async move {
            let _result = listener.start_device_removed_listener(sender).await;
        });

        while let Ok(response) = receiver.recv().await {
            println!("Response: {}", response);
        }
        println!("Listener done!");
    }

    #[tokio::test]
    async fn test_connected_listener() {
        let bt = Bluetooth::new(None)
            .await
            .expect("Failed to initialize Bluetooth");

        let bt_listener = Listener::new(bt.clone())
            .await
            .expect("Failed to initialize BluetoothListener");

        start_device_listener(bt_listener).await;
    }

    async fn start_device_listener(listener: Listener) {
        let (sender, receiver) = async_channel::bounded(1);

        tokio::spawn(async move {
            let _result = listener
                .start_device_listener(String::from("64:A2:F9:F2:FA:EC"), sender)
                .await;
        });

        while let Ok(response) = receiver.recv().await {
            println!("Response: {}", response);
        }

        println!("Listener done!");
    }
}
