use dbus::arg::PropMap;
use serde::Serialize;

use crate::utils::{
    get_bool_from_variant, get_int_from_variant, get_string_array_from_variant,
    get_string_from_variant,
};

#[derive(Default, Debug, Serialize)]
pub struct Status {
    pub address: Option<String>,
    pub address_type: Option<String>,
    pub alias: Option<String>,
    pub class: Option<i64>,
    pub connected_devices: Option<i64>,
    pub discoverable: Option<bool>,
    pub discoverable_timeout: Option<i64>,
    pub discovering: Option<bool>,
    pub manufacturer: Option<i64>,
    pub modalias: Option<String>,
    pub name: Option<String>,
    pub pairable: Option<bool>,
    pub pairable_timeout: Option<i64>,
    pub powered: Option<bool>,
    pub power_state: Option<String>,
    pub roles: Option<Vec<String>>,
    pub uuids: Option<Vec<String>>,
    pub version: Option<i64>,
}

impl Status {
    pub fn new(status: PropMap) -> Self {
        Self {
            address: Some(get_string_from_variant(status.get("Address"))),
            address_type: Some(get_string_from_variant(status.get("AddressType"))),
            alias: Some(get_string_from_variant(status.get("Alias"))),
            class: Some(get_int_from_variant(status.get("Class"))),
            connected_devices: Some(get_int_from_variant(status.get("ConnectedDevices"))),
            discoverable: Some(get_bool_from_variant(status.get("Discoverable"))),
            discoverable_timeout: Some(get_int_from_variant(status.get("DiscoverableTimeout"))),
            discovering: Some(get_bool_from_variant(status.get("Discovering"))),
            manufacturer: Some(get_int_from_variant(status.get("Manufacturer"))),
            modalias: Some(get_string_from_variant(status.get("Modalias"))),
            name: Some(get_string_from_variant(status.get("Name"))),
            pairable: Some(get_bool_from_variant(status.get("Pairable"))),
            pairable_timeout: Some(get_int_from_variant(status.get("PairableTimeout"))),
            powered: Some(get_bool_from_variant(status.get("Powered"))),
            power_state: Some(get_string_from_variant(status.get("PowerState"))),
            roles: Some(get_string_array_from_variant(status.get("Roles"))),
            uuids: Some(get_string_array_from_variant(status.get("UUIDs"))),
            version: Some(get_int_from_variant(status.get("Version"))),
        }
    }
}
