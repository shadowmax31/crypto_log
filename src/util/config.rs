use std::path::PathBuf;

use configparser::ini::Ini;

pub struct Config {
    ini: Ini
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let path;

        if cfg!(debug_assertions) {
            path = PathBuf::from("crypto.ini");
        }
        else {
            path = Self::get_home_dir()?.join(".config/crypto/crypto.ini");
        }
        

        let mut init = Ini::new();
        init.load(path)?;

        let config = Config {
            ini: init
        };

        Ok(config)
    }

    pub fn db_path(&self) -> Result<String, String> {
        let path = self.ini.get("general", "database");

        let path = match path {
            Some(mut p) => {
                if p.starts_with("~") {
                    let home = Self::get_home_dir()?;
                    let home = match home.to_str() {
                        Some(h) => h,
                        None => return Err("Missing home path".to_string())
                    };

                    p = p.replace("~", home);
                }

                p
            },
            None => return Err("Missing database path from the config file".to_owned())
        };

        Ok(path)
    }


    pub fn date_format(&self) -> Result<String, String> {
        match self.ini.get("general", "dateFormat") {
            Some(v) => Ok(v),
            None => Err("Missing date format from config file".to_owned())
        }
    }


    // pub fn coin_market_cap_key(&self) {
        // return self.config["general"]["coinMarketCapKey"]
    // }


    // pub fn currency(&self) {
        // return self.config["general"]["currency"]
    // }

    fn get_home_dir() -> Result<PathBuf, String> {
        match home::home_dir() {
            Some(path) => Ok(path),
            None => Err("Cannot find the home directory".to_owned())
        }
    }

}