from setuptools import setup

setup(
    name="crypto",
    version="0.0.4",
    description="Command line tool to manage crypto transactions",
    license = "MIT",
    url = "https://github.com/shadowmax31/crypto_log",
    package_dir={"": "src"},
    scripts=["src/crypto"],
    install_requires = [
        "fire",
        "tinydb",
        "tinydb-serialization"
    ]
)

