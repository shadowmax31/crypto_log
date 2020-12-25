class CapitalGain:
    costBasis = None
    amount = None
    marketPrice = None

    def __init__(self, db, obj):
        self.db = db
        self.costBasis = obj["cost_basis"]
        self.amount = obj["amount"]
        self.marketPrice = obj["market_price"]


    def calculatePurchasePrice(self):
        return self.costBasis * self.amount
    

    def gain(self):
        return round(self.marketPrice - self.calculatePurchasePrice(), 2)

