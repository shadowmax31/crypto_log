from const import TransactionType, Tables
from undo import Undo
from config import Config

from datetime import datetime
from decimal import Decimal


class Transaction:

    def __init__(self, db):
        self.db = db
        self.undo = Undo(self.db)
        self.config = Config()

    def buy(self, date, amount, ticker, price, description):
        docId = None

        if not self.transactionExists(date, ticker):
            table = self.db.table(ticker)
            docId = table.insert(self.createTransaction(date, amount, price, description, TransactionType.BUY.name))
            self.undo.save(ticker, Undo.INSERT, docId)

        return docId

    
    def sell(self, date, amount, ticker, price, description):
        docId = None

        if not self.transactionExists(date, ticker):
            table = self.db.table(ticker)
            docId = table.insert(self.createTransaction(date, amount, price, description, TransactionType.SELL.name))
            self.undo.save(ticker, Undo.INSERT, docId)
    
        return docId


    def exchange(self, date, fromAmount, fromTicker, toAmount, toTicker, toPrice, description):
        if description is None:
            description = ""
        else:
            description = description + " / "

        self.sell(date, fromAmount, fromTicker, toPrice, description + "Exchanged for " + toTicker)
        self.buy(date, toAmount, toTicker, toPrice, description + "Bought with " + fromTicker)
        self.undo.save(None, Undo.SKIP, None, 3)


    def createTransaction(self, date, amount, price, description, tType):
        return {
            "date": self.convertStrToDate(date),
            "amount": Decimal(str(amount)),
            "price": Decimal(str(price)),
            "description": description,
            "type": tType
            }


    def transactionExists(self, date, ticker, silent=False):
        date = self.convertStrToDate(date)
        table = self.db.table(ticker)

        exists = False
        for row in table:
            if row["date"] == date:
                exists = True
                break

        if exists and not silent:
            print("The transaction already exists (" + ticker + " : " + str(date) + ")")

        return exists

    def convertStrToDate(self, date):
        if isinstance(date, str):
            date = datetime.strptime(date, self.config.dateFormat())

        return date

