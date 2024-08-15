use thiserror::Error;

#[derive(Error, Debug)]
pub enum BluetoothError {
    #[error("DBus connection error: {0}")]
    DBusConnectionError(#[from] dbus::Error),
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Request timeout")]
    Timeout,

    #[error("Failed to start discovery")]
    StartDiscoveryFailed,
    #[error("Failed to stop discovery")]
    StopDiscoveryFailed,

    #[error("Invalid device")]
    InvalidDevice,

    #[error("Failed to connect to device: {0}")]
    ConnectionFailed(String),

    #[error("Unknown property: {0}")]
    UnknwonProperty(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
    //Add others
}
