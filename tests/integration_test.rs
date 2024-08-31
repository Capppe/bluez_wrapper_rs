extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bt_wrapper::bluetooth::{Bluetooth, Power};
    use tokio::sync::oneshot;

    #[tokio::test]
    async fn test_start_discovery() {
        let bt = Bluetooth::new(None)
            .await
            .expect("Failed to initialize Bluetooth");

        let result = bt
            .start_discovery(Duration::from_secs(5))
            .await
            .expect("Failed to discover devices");

        assert!(result == ());
    }

    #[tokio::test]
    async fn test_stop_discovery() {
        let bt = Bluetooth::new(None)
            .await
            .expect("Failed to initialize Bluetooth");

        let result = bt.stop_discovery().await.expect("Failed to stop discovery");

        assert!(result == ());
    }

    #[tokio::test]
    async fn test_stop_discovery_while_running() {
        let bt = Bluetooth::new(None)
            .await
            .expect("Failed to initialize Bluetooth");

        let bt_clone = bt.clone();
        let (tx, rx) = oneshot::channel();

        let discovery_thread = tokio::spawn(async move {
            let discovery_future = bt_clone.start_discovery(Duration::from_secs(15));

            tokio::select! {
            _ = discovery_future => {
                    println!("Discovery complete");
                }
            _ = rx => {
                    println!("Discovery cancelled");
                }
            }
        });

        tokio::time::sleep(Duration::from_secs(3)).await;
        let _ = tx.send(());

        discovery_thread
            .await
            .expect("Failed to join discovery thread!");
    }

    #[tokio::test]
    async fn test_power_off() {
        let bt = Bluetooth::new(None)
            .await
            .expect("Failed to initialize Bluetooth");

        let result = bt.power_off().await;

        assert!(result == Ok(()));
    }

    #[tokio::test]
    async fn test_power_on() {
        let bt = Bluetooth::new(None)
            .await
            .expect("Failed to initialize Bluetooth");

        let result = bt.power_on().await;

        assert!(result == Ok(()));
    }
}
