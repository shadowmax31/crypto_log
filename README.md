# crypto_log
This is a cli-tool to log and track the Cost Basis and Capital Gain when transacting with CryptoCurrency

THIS IS A WORK IN PROGRESS

# Installation

> python setup.py bdist_wheel
>
> pip install -e .

# TODO

Testing
- IMPORTANT Add unit tests

Base sytem
- Since we have to work with amounts that have many decimals (0.031235 btc), we should find a way to use the Decimal class for better accuracy;
- Review code;
- Change the table structure. Create a ticker table and put everything in it. With this change, we could remove the history table/class and loop through the ticker table to export all the useful data;
- Find a way to insert data in any order. At the moment, buys and sells have to be entered in the right order;
- Have a way to change the amount of crypto without changing the cost basis (would be useful to take into account the fees when transfering crypto between wallets);
