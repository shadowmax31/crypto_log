from const import TransactionType
from cost_basis import CostBasis
from undo import Undo

class Transaction:

    def __init__(self, db):
        self.db = db
        self.undo = Undo(self.db)

    def buy(self, amount, ticker, price, description=None):
        table = self.db.table(ticker)
        docId = table.insert(self.createTransaction(amount, price, description, TransactionType.BUY.name))
        self.undo.save(ticker, Undo.INSERT, docId)

    def sell(self, amount, ticker, price, description=None):
        cost = CostBasis(self.db)
        tickerBasis = cost.calculate(ticker)

        # For the cost basis, selling is the same as buying negative amount
        table = self.db.table(ticker)
        tickerTableId = table.insert(self.createTransaction(amount, price, description, TransactionType.SELL.name))

        table = self.db.table("capital_gain")
        capitalGainId = table.insert({
            "cost_basis": tickerBasis,
            "amount": amount,
            "market_price": price
            })
        self.undo.save(ticker, Undo.INSERT, tickerTableId)
        self.undo.save("capital_gain", Undo.INSERT, capitalGainId, 2)

    def exchange(self, fromAmount, fromTicker, toAmount, toTicker, toPrice):
        cost = CostBasis(self.db)
        tickerBasis = cost.calculate(fromTicker)

        self.sell(fromAmount, fromTicker, toPrice, "Exchanged for " + toTicker)
        self.buy(toAmount, toTicker, toPrice, "Bought with " + fromTicker)
        self.undo.save(None, Undo.SKIP, None, 3)

    def createTransaction(self, amount, price, description, tType):
        return {
            "amount": amount,
            "price": price,
            "description": description,
            "type": tType
            }

