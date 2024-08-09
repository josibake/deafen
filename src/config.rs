use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db_path: PathBuf,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_from_env() {
        env::set_var("DB_PATH", "/tmp/test.db");
        env::set_var("PORT", "8080");

        let config = Config::from_env().unwrap();
        assert_eq!(config.db_path, PathBuf::from("/tmp/test.db"));
        assert_eq!(config.port, 8080);
    }
}
