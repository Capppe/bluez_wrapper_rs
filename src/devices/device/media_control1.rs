use dbus::blocking::SyncConnection;

use crate::{utils::get_path_from_address, DBusItem, DBusProxy, Methods, Properties};

pub struct MediaControl1 {
    interface: String,
    object_path: String,
    connection: SyncConnection,
}

impl DBusItem for MediaControl1 {
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

impl<'a> DBusProxy<'a> for MediaControl1 {}

impl Methods for MediaControl1 {}

impl Properties for MediaControl1 {}

impl MediaControl1 {
    pub fn new(dev_address: &str) -> Result<Self, dbus::Error> {
        let dev_address = get_path_from_address(dev_address, "/org/bluez/hci0");
        Ok(Self {
            interface: "org.bluez.MediaControl1".to_string(),
            object_path: format!("{}", dev_address),
            connection: SyncConnection::new_system()?,
        })
    }

    // Methods
    pub fn fast_forward(&self) -> Result<(), String> {
        self.call_method_no_return("FastForward", ())
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

    pub fn previous(&self) -> Result<(), String> {
        self.call_method_no_return("Previous", ())
    }

    pub fn rewind(&self) -> Result<(), String> {
        self.call_method_no_return("Rewind", ())
    }

    pub fn stop(&self) -> Result<(), String> {
        self.call_method_no_return("Stop", ())
    }

    pub fn volume_down(&self) -> Result<(), String> {
        self.call_method_no_return("VolumeDown", ())
    }

    pub fn volume_up(&self) -> Result<(), String> {
        self.call_method_no_return("VolumeUp", ())
    }
}
