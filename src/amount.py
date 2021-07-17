from const import TransactionType

class Amount:

    def __init__(self, db):
        self.db = db


    def calculate(self, ticker):
        table = self.db.table(ticker)

        amount = 0
        for row in table:
            if row["type"] == TransactionType.BUY.name:
                amount += row["amount"]
            elif row["type"] == TransactionType.SELL.name:
                amount -= row["amount"]
            else:
                raise Exception("Transaction type [" + row["type"] + "] not implemented") 

        return amount
