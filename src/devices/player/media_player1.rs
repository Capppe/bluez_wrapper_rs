use dbus::blocking::SyncConnection;

use crate::{
    utils::{get_path_from_address, validate_address},
    DBusItem, DBusProxy, Methods, Properties,
};

pub struct MediaPlayer1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for MediaPlayer1 {
    fn get_interface(&self) -> &str {
        &self.interface
    }

    fn get_object_path(&self) -> &str {
        &self.object_path
    }

    fn get_connection(&self) -> &SyncConnection {
        &self.connection
    }
}

impl<'a> DBusProxy<'a> for MediaPlayer1 {}

impl Methods for MediaPlayer1 {}

impl Properties for MediaPlayer1 {}

impl MediaPlayer1 {
    pub fn new(dev_address: &str) -> Result<Self, String> {
        if !validate_address(dev_address) {
            return Err(format!("Invalid address: {}", dev_address));
        };

        let dev_address = get_path_from_address(dev_address, "/org/bluez/hci0");
        Ok(Self {
            interface: "org.bluez.MediaPlayer1".to_string(),
            object_path: format!("{}", dev_address),
            connection: SyncConnection::new_system()
                .map_err(|e| format!("Failed to acquire dbus-connection: {}", e))?,
        })
    }

    // Methods
    pub fn fast_forward(&self) -> Result<(), String> {
        self.call_method_no_return("FastForward", ())
    }

    pub fn hold(&self, avc_key: u8) -> Result<(), String> {
        self.call_method_no_return("Hold", (avc_key,))
    }

    pub fn next(&self) -> Result<(), String> {
        self.call_method_no_return("Next", ())
    }

    pub fn pause(&self) -> Result<(), String> {
        self.call_method_no_return("Pause", ())
    }

    pub fn play(&self) -> Result<(), String> {
        self.call_method_no_return("Play", ())
    }

    pub fn press(&self, avc_key: u8) -> Result<(), String> {
        self.call_method_no_return("Press", (avc_key,))
    }

    pub fn previous(&self) -> Result<(), String> {
        self.call_method_no_return("Previous", ())
    }

    pub fn release(&self) -> Result<(), String> {
        self.call_method_no_return("Release", ())
    }

    pub fn rewind(&self) -> Result<(), String> {
        self.call_method_no_return("Rewind", ())
    }

    pub fn stop(&self) -> Result<(), String> {
        self.call_method_no_return("Stop", ())
    }
}
