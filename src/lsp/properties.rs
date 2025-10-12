use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ServerInfo {
    name: &'static str,
    version: &'static str,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
        }
    }
}
