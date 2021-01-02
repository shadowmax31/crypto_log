from const import TransactionType
from cost_basis import CostBasis
from undo import Undo

class Transaction:

    def __init__(self, db):
        self.db = db
        self.undo = Undo(self.db)

    def buy(self, date, amount, ticker, price, description):
        table = self.db.table(ticker)
        docId = table.insert(self.createTransaction(date, amount, price, description, TransactionType.BUY.name))
        self.undo.save(ticker, Undo.INSERT, docId)

    def sell(self, date, amount, ticker, price, description):
        cost = CostBasis(self.db)
        tickerBasis = cost.calculate(ticker)

        # For the cost basis, selling is the same as buying negative amount
        table = self.db.table(ticker)
        tickerTableId = table.insert(self.createTransaction(date, amount, price, description, TransactionType.SELL.name))

        table = self.db.table("capital_gain")
        capitalGainId = table.insert({
            "cost_basis": tickerBasis,
            "amount": amount,
            "market_price": price
            })
        self.undo.save(ticker, Undo.INSERT, tickerTableId)
        self.undo.save("capital_gain", Undo.INSERT, capitalGainId, 2)

    def exchange(self, date, fromAmount, fromTicker, toAmount, toTicker, toPrice, description):
        cost = CostBasis(self.db)
        tickerBasis = cost.calculate(fromTicker)

        if description is None:
            description = ""
        else:
            description = description + " / "

        self.sell(date, fromAmount, fromTicker, toPrice, description + "Exchanged for " + toTicker)
        self.buy(date, toAmount, toTicker, toPrice, description + "Bought with " + fromTicker)
        self.undo.save(None, Undo.SKIP, None, 3)

    def createTransaction(self, date, amount, price, description, tType):
        return {
            "date": date,
            "amount": amount,
            "price": price,
            "description": description,
            "type": tType
            }

