use dbus::{Message, Path};
use regex::Regex;
use std::collections::HashMap;

pub fn get_path_from_address(address: &str, adapter_path: &str) -> String {
    return format!("{}/dev_{}", adapter_path, address.replace(":", "_"));
}

pub fn get_address_from_path(path: Path) -> Result<String, String> {
    if let Some(s) = path.split("dev_").last() {
        Ok(s.replace("_", ":"))
    } else {
        Err(format!("Invalid device path: {}", path))
    }
}

pub fn validate_path(path: &str) -> bool {
    let pattern = r"^/org/bluez/hci\d+/dev(_[A-Fa-f0-9]{2}){6}$";
    if let Ok(re) = Regex::new(pattern) {
        return re.is_match(path);
    };
    false
}

pub fn validate_address(address: &str) -> bool {
    let pattern = r"^([A-Fa-f0-9]{2}:){5}[A-Fa-f0-9]{2}$";
    if let Ok(re) = Regex::new(pattern) {
        return re.is_match(address);
    };

    false
}

pub fn read_dbus_propmap(
    message: Message,
) -> Result<(dbus::Path<'static>, HashMap<String, dbus::arg::PropMap>), String> {
    return match message.read2::<dbus::Path<'static>, HashMap<String, dbus::arg::PropMap>>() {
        Ok(res) => Ok(res),
        Err(e) => Err(format!("Failed to parse propmap, cause: {}", e)),
    };
}
