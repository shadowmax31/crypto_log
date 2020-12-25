from cost_basis import CostBasis

class Transaction:
    def __init__(self, db):
        self.db = db

    def buy(self, amount, ticker, price, description=None):
        table = self.db.table(ticker)
        table.insert({
            "amount": amount,
            "price": price,
            "description": description
            })

    def sell(self, amount, ticker, forPrice, description="Selling"):
        cost = CostBasis(self.db)
        tickerBasis = cost.calculate(ticker)
        # get the cost basis
        # For the cost basis, selling is the same as buying negative amount
        self.buy(-amount, ticker, -forPrice, description)

        table = self.db.table("capital_gain")
        table.insert({
            "cost_basis": tickerBasis,
            "amount": amount,
            "market_price": forPrice
            })

    def exchange(self, fromAmount, fromTicker, toAmount, toTicker, toPrice):
        cost = CostBasis(self.db)
        tickerBasis = cost.calculate(fromTicker)

        self.sell(fromAmount, fromTicker, toPrice, "Exchange for " + toTicker)
        self.buy(toAmount, toTicker, toPrice)


