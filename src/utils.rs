use dbus::{
    arg::{RefArg, Variant},
    Message, Path,
};
use std::collections::HashMap;

pub fn get_string_from_variant(variant: Option<&Variant<Box<dyn RefArg>>>) -> String {
    if let Some(s) = variant {
        if let Some(value) = s.0.as_str() {
            return value.to_string();
        }
    }

    String::new()
}

pub fn get_int_from_variant(variant: Option<&Variant<Box<dyn RefArg>>>) -> i64 {
    if let Some(u) = variant {
        if let Some(value) = u.0.as_i64() {
            return value;
        }
    }

    return 0;
}

pub fn get_bool_from_variant(variant: Option<&Variant<Box<dyn RefArg>>>) -> bool {
    if let Some(b) = variant {
        if let Some(value) = b.0.as_u64() {
            return value == 1;
        }
    }

    return false;
}

pub fn get_string_array_from_variant(variant: Option<&Variant<Box<dyn RefArg>>>) -> Vec<String> {
    let mut strings = Vec::new();
    if let Some(v) = variant {
        if let Some(values) = v.0.as_iter() {
            for value in values {
                if let Some(s) = value.as_str() {
                    strings.push(s.to_string());
                }
            }
        }
    }
    return strings;
}

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

pub fn read_dbus_propmap(
    message: Message,
) -> Result<(dbus::Path<'static>, HashMap<String, dbus::arg::PropMap>), String> {
    return match message.read2::<dbus::Path<'static>, HashMap<String, dbus::arg::PropMap>>() {
        Ok(res) => Ok(res),
        Err(e) => Err(format!("Failed to parse propmap, cause: {}", e)),
    };
}
