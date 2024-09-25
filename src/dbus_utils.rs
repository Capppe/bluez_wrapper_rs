use std::collections::HashMap;

use dbus::{
    arg::{PropMap, RefArg, Variant},
    blocking::{Proxy, SyncConnection},
};

pub fn create_proxy<'a>(
    dest: Option<&'a str>,
    path: &'a str,
    timeout: std::time::Duration,
    conn: &'a SyncConnection,
) -> Result<Proxy<'a, &'a SyncConnection>, String> {
    Ok(Proxy::new(dest.unwrap_or("org.bluez"), path, timeout, conn))
}

pub fn parse_propmap<'a>(props: &PropMap) -> HashMap<String, String> {
    props
        .into_iter()
        .map(|(key, value)| {
            let value_as_string = variant_to_string(value);
            (key.clone(), value_as_string.trim().to_owned())
        })
        .collect()
}

pub fn parse_variant<R: 'static>(variant: &Variant<Box<dyn RefArg>>) -> Result<&R, String> {
    let ref_arg = variant.0.as_ref();

    if let Some(r) = ref_arg.as_any().downcast_ref::<R>() {
        Ok(r)
    } else {
        Err(format!("Failed to convert {:?} to specified type", variant))
    }
}

fn variant_to_string<T: std::fmt::Debug>(variant: &Variant<T>) -> String {
    format!("{:?}", variant)
}
