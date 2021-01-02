# crypto_log
This is a cli-tool to log and track the Cost Basis and Capital Gain when transacting with CryptoCurrency

THIS IS A WORK IN PROGRESS

# Installation

> python setup.py bdist_wheel
>
> pip install -e .

# TODO
Base sytem
- Since we have to work with amounts that have many decimals (0.031235 btc), we should find a way to use the Decimal class for better accuracy
- Review code
- Find a way to record when a capital gain/loss has been reported in a tax report

Reports
- Give more info when using the details tag with the cg (Capital gain) report

Testing
- IMPORTANT Add unit tests


