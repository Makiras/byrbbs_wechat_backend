use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConf,
    pub key: KeyConf,
}

#[derive(Debug, Deserialize)]
pub struct ServerConf {
    pub host_ip: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct KeyConf {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

pub fn read_config(path: &Path) -> Result<Config, anyhow::Error> {
    let mut buf: BufReader<File> = match File::open(&path) {
        Ok(file) => {
            println!("Reading config from {}", path.display());
            BufReader::new(file)
        }
        Err(e) => {
            println!("Failed to read config from {}, error {}", path.display(), e);
            return Err(anyhow::Error::new(e));
        }
    };
    let mut conf_str = String::new();
    match buf.read_to_string(&mut conf_str) {
        Ok(_) => {
            let conf: Config = match toml::from_str(&conf_str) {
                Ok(conf) => conf,
                Err(e) => {
                    println!(
                        "Failed to parse config from {}, error {}",
                        path.display(),
                        e
                    );
                    return Err(anyhow::Error::new(e));
                }
            };
            return Ok(conf);
        }
        Err(e) => {
            println!("Failed to read config from {}, error {}", path.display(), e);
            return Err(anyhow::Error::new(e));
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ServerConf: \n{:}KeyConf:\n{:}",
            &self.server, &self.key
        )
    }
}

impl fmt::Display for ServerConf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "host_ip: {:?},\nport: {:?}\n", &self.host_ip, &self.port)
    }
}

impl fmt::Display for KeyConf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "client_id: {:?},\nclient_secret: **********\n",
            &self.client_id
        )
    }
}
