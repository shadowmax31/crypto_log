import requests

from decimal import Decimal
from config import Config

class CryptoApi:
    def __init__(self):
        self.config = Config()
        self.baseUrl = "https://pro-api.coinmarketcap.com"


    def getPrice(self, ticker):
        currency = self.config.currency()
        query = "/v1/cryptocurrency/quotes/latest?symbol=" + ticker + "&convert=" + currency

        r = requests.get(self.baseUrl + query, headers={"X-CMC_PRO_API_key": self.config.coinMarketCapKey()})

        price = None
        hasValue = False
        if r.json()["status"]["error_code"] == 0:
            info = r.json()["data"][ticker]
            if info["is_active"] == 1:
                price = info["quote"][currency]["price"]
                price = Decimal(price)
                hasValue = True

        if not hasValue: 
            raise Exception("The price for cannot be found for [" + ticker + "]")

        return price

