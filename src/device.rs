#[derive(Debug)]
pub struct Device {
    pub name: Option<String>,
    pub address: Option<String>,
    // r#type: Type,
    pub paired: Option<bool>,
    pub connected: Option<bool>,
    pub icon: Option<String>,
}

// #[derive(Debug)]
// enum Type {
//     Phone,
//     PC,
// }
