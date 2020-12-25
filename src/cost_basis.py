from const import RED, ENDC

class CostBasis:
    def __init__(self, db):
        self.db = db

    def calculate(self, ticker, printDetails=False):
        table = self.db.table(ticker)        

        amount = 0
        price = 0
        for row in table:
            amount += row["amount"]
            price += row["price"]

            if printDetails:
                print(str(row.doc_id) + ": " + str(row["amount"]) + " " + ticker + " for " + str(row["price"]) + "$")


        costBasis = None
        if amount > 0:
            costBasis = round(price / amount, 2)
        elif amount < 0:
            print(RED + "You have " + str(amount) + " amount of " + ticker + ". Please check your data." + ENDC)

        if printDetails and len(table) > 0:
            print("")

        return costBasis
