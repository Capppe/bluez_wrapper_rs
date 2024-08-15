use std::{collections::HashMap, fmt::Display};

use dbus::arg::{RefArg, Variant};
use serde::{Deserialize, Serialize};

use crate::errors::BluetoothError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub name: Option<String>,
    pub address: Option<String>,
    pub paired: Option<bool>,
    pub connected: Option<bool>,
    pub icon: Option<String>,
}

pub trait Json {
    fn to_json(&self) -> Result<String, serde_json::Error>;
    fn from_json(&self) -> Result<Device, serde_json::Error>;
}

impl Device {
    pub fn new(n: String, a: String, p: bool, c: bool, i: String) -> Self {
        Self {
            name: Some(n),
            address: Some(a),
            paired: Some(p),
            connected: Some(c),
            icon: Some(i),
        }
    }

    pub fn new_from_dbus_message(
        message: HashMap<String, Variant<Box<dyn RefArg>>>,
    ) -> Result<Self, BluetoothError> {
        fn extract_string(value: &Variant<Box<dyn RefArg>>) -> Result<String, BluetoothError> {
            value
                .0
                .as_str()
                .map(String::from)
                .ok_or(BluetoothError::InvalidDevice)
        }

        fn extract_bool(value: &Variant<Box<dyn RefArg>>) -> Result<bool, BluetoothError> {
            dbus::arg::cast::<bool>(&value.0)
                .ok_or(BluetoothError::InvalidDevice)
                .copied()
        }

        let mut name = None;
        let mut address = None;
        let mut paired = None;
        let mut connected = None;
        let mut icon = None;

        for (key, value) in message {
            println!("Key: {} - Value: {:?}", key, value);
            match key.as_str() {
                "Name" => name = Some(extract_string(&value)?),
                "Address" => address = Some(extract_string(&value)?),
                "Paired" => paired = Some(extract_bool(&value)?),
                "Connected" => connected = Some(extract_bool(&value)?),
                "Icon" => icon = Some(extract_string(&value)?),
                &_ => {}
            }
        }
        Ok(Self {
            name,
            address,
            paired,
            connected,
            icon,
        })
    }
}

impl Json for Device {
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    fn from_json(&self) -> Result<Device, serde_json::Error> {
        serde_json::from_str(&self.to_string())
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {:?}, Address: {:?}, Paired: {:?}, Connected: {:?}, Icon: {:?}",
            self.name, self.address, self.paired, self.connected, self.icon
        )
    }
}
