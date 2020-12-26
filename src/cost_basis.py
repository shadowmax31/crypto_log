from const import RED, ENDC, TransactionType

class CostBasis:
    def __init__(self, db):
        self.db = db

    def calculate(self, ticker, printDetails=False):
        table = self.db.table(ticker)        

        amount = 0
        totalCost = 0
        for row in table:
            if row["type"] == TransactionType.BUY.name:
                amount += row["amount"]
                totalCost += row["price"]
            elif row["type"] == TransactionType.SELL.name:
                perUnitCost = 0
                if amount > 0:
                    perUnitCost = totalCost / amount

                amount -= row["amount"]
                totalCost -= (perUnitCost * row["amount"])
            else:
                raise Exception("Transaction type [" + row["type"] + "] not implemented") 

            if printDetails:
                msg = str(row.doc_id) + ": "+ row["type"] + " "  + str(row["amount"]) + " " + ticker + " for " + str(row["price"]) + "$"
                if row["description"] is not None:
                    msg += " (" + row["description"] + ")"

                print(msg)

                if amount == 0:
                    print("----- No more " + ticker + " -----")
                    print("")

        costBasis = None
        if amount > 0:
            costBasis = round(totalCost / amount, 2)
        elif amount < 0:
            print(RED + "You have " + str(amount) + " amount of " + ticker + ". Please check your data." + ENDC)

        if printDetails and len(table) > 0:
            print("")

        return costBasis
