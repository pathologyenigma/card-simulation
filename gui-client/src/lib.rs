use std::io::Read;

use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct ApplicationConfig {
    pub(crate) window: WindowConfig,
    pub(crate) web: WebConfig
}

#[derive(Deserialize, Debug)]
pub struct WindowConfig {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) resizable: bool,
    pub(crate) title: String,
}

#[derive(Deserialize, Debug)]
pub struct WebConfig {
    pub(crate) url: String,
    pub(crate) root_path: String,
    pub(crate) ws_path: String,
}

pub fn parse_config() -> std::io::Result<ApplicationConfig> {
    let mut config_file = std::fs::File::open("config.toml")?;
    let mut str = String::new();
    config_file.read_to_string(&mut str)?;
    Ok(toml::from_str(&str).expect("failed to parse config file, check that you wrote it correctly"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read() {
        println!("{:?}", parse_config().unwrap());
    }
}