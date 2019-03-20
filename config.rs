use serde_derive::{Deserialize};

#[derive(Deserialize)]
pub struct Config  {
    pub app: AppConfig,
    pub network: NetworkConfig,
}

impl Config {

    pub fn new() -> Config {
        Config {
            app: AppConfig::new(),
            network: NetworkConfig::new(),
        }
    }

    pub fn load(path: String) -> Config {
        match super::toml_parser::parse_toml::<Config>(path) {
            Ok(conf) => conf,
            Err(err) => {
                println!("Could not parse config file ({})\nBuild default one.", err.to_string());
                Config::new()
            }
        }
    }

}

#[derive(Deserialize)]
pub struct AppConfig {
    pub version: f32,
    pub name: String,
    pub prod: bool,
}

impl AppConfig {

    pub fn new() -> AppConfig {
        AppConfig {
            version: 0.1,
            name: String::from("John Doe Application"),
            prod: false
        }
    }

}

#[derive(Deserialize)]
pub struct NetworkConfig {
    pub address: String,
}

impl NetworkConfig {

    pub fn new() -> NetworkConfig {
        NetworkConfig {
            address: String::from("127.0.0.1:4242"),
        }
    }

}
