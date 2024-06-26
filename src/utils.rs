use crate::device::Device;

use dbus::arg::{RefArg, Variant};
use std::collections::HashMap;

pub fn get_device_properties(properties: &HashMap<String, Variant<Box<dyn RefArg>>>) -> Device {
    let name = properties
        .get("Name")
        .and_then(|v| v.0.as_str().map(|s| s.to_string()));
    let address = properties
        .get("Address")
        .and_then(|v| v.0.as_str().map(|s| s.to_string()));
    let paired = properties
        .get("Paired")
        .and_then(|v| v.as_u64().map(|c| c == 1));
    let connected = properties
        .get("Connected")
        .and_then(|v| v.as_u64().map(|c| c == 1));
    let icon = properties
        .get("Icon")
        .and_then(|v| v.0.as_str().map(|s| s.to_string()));

    Device {
        name,
        address,
        paired,
        connected,
        icon,
    }
}
