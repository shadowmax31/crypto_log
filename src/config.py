from pathlib import Path
from configparser import ConfigParser

class Config:
    def __init__(self):
        self.config = ConfigParser()
        path = str(Path.home()) + "/.crypto/crypto.ini"
        self.config.read(path)

    def dbPath(self):
        path = self.config["general"]["database"]

        if path.startswith("~"):
            path = path.replace("~", str(Path.home()))

        return path
