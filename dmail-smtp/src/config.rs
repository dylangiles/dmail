use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

use crate::smtp::SmtpReply;

#[derive(Debug, Deserialize)]
pub(crate) struct SmtpConfig {
    #[serde(default = "default_smtp_port")]
    pub(crate) port: u16,

    #[serde(default)]
    pub(crate) reply_messages: HashMap<SmtpReply, String>,
}

impl<'a> Default for SmtpConfig {
    fn default() -> Self {
        Self {
            port: default_smtp_port(),
            reply_messages: HashMap::default(),
        }
    }
}
fn default_smtp_port() -> u16 {
    DEFAULT_SMTP_PORT
}

const DEFAULT_SMTP_PORT: u16 = 25;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) smtp: SmtpConfig,
}

impl Config {
    pub(crate) fn load() -> Result<Self, Box<dyn Error>> {
        Self::from_file(Path::new("./dmail.toml"))
    }

    pub(crate) fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        match toml::de::from_str::<Config>(buf.as_str()) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            smtp: Default::default(),
        }
    }
}
