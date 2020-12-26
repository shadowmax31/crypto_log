# crypto_log
This is a cli-tool to log and track the Cost Basis and Capital Gain when transacting with CryptoCurrency

THIS IS A WORK IN PROGRESS

# TODO
Base sytem
- Manage dates for the transactions
- Allow some sort of undo, in case a transaction is added by mistake. At the moment, we have to delete the corresponding entries in the ticker table and in the captial_gain table
- Since we have to work with amounts that have many decimals (0.031235 btc), we should find a way to use the Decimal class for better accuracy

Reports
- Give more info when using the details tag with the cg (Capital gain) report

Testing
- Add unit tests
