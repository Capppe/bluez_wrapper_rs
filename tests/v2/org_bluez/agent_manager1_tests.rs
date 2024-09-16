extern crate bt_wrapper;

#[cfg(test)]
mod tests {
    use bt_wrapper::org_bluez::agent_manager1::AgentManager1;
    use dbus::Path;

    #[tokio::test]
    async fn test_method_register_agent() {
        let iface = AgentManager1::new().unwrap();

        let res = iface
            .register_agent(Path::from("/test"), "".to_owned())
            .unwrap();

        assert!(res == ())
    }

    // Fails, find out how to use
    #[tokio::test]
    async fn test_method_request_default_agent() {
        let iface = AgentManager1::new().unwrap();

        let res = iface.request_default_agent(Path::from("/test")).unwrap();

        assert!(res == ())
    }

    // Fails, find out how to use
    #[tokio::test]
    async fn test_method_unregister_agent() {
        let iface = AgentManager1::new().unwrap();

        let res = iface.unregister_agent(Path::from("/test")).unwrap();

        assert!(res == ())
    }
}
