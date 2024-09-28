extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use bt_wrapper::properties::Properties;

    #[tokio::test]
    async fn test_listen_properties_changed() {
        let iface = Properties::new("/org/bluez/hci0").unwrap();

        let (tx, mut rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            let _ = iface.properties_changed(tx, None).await;
        });

        while let Some(resp) = rx.recv().await {
            println!("Resp: {:?}", resp);
        }

        assert!(1 == 1)
    }
}
