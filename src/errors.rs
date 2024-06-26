use thiserror::Error;

#[derive(Error, Debug)]
pub enum BluetoothError {
    #[error("DBus connection error: {0}")]
    DBusConnectionError(#[from] dbus::Error),
    #[error("Device not found")]
    DeviceNotFound,
    //Add others
}
