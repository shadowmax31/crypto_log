// Class that read the config file
impl Config {
    pub fn new() -> Config {
        // self.config = ConfigParser()
        // path = str(Path.home()) + "/.crypto/crypto.ini"
        // self.config.read(path)
    }

    pub fn dbPath(&self) {
        // path = self.config["general"]["database"]

        // if path.startswith("~"):
        //     path = path.replace("~", str(Path.home()))

        // return path
    }


    pub fn dateFormat(&self) {
        // return self.config["general"]["dateFormat"]
    }


    pub fn coinMarketCapKey(&self) {
        // return self.config["general"]["coinMarketCapKey"]
    }


    pub fn currency(&self) {
        // return self.config["general"]["currency"]
    }

}