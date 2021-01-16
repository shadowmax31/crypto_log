from const import RED, ENDC, BOLD, TransactionType
from config import Config

from operator import itemgetter

class CostBasis:
    def __init__(self, db):
        self.db = db
        self.config = Config()

    def calculate(self, ticker, stopAt=None, printDetails=False):
        table = self.db.table(ticker)        

        amount = 0
        totalCost = 0

        for row in sorted(table, key=itemgetter("date")):
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

            self.printDetails(printDetails, amount, ticker, row)
                    
            if stopAt == row.doc_id:
                break

        costBasis = None
        if amount > 0:
            costBasis = round(totalCost / amount, 4)
        elif amount < 0:
            print(BOLD + RED + "You have " + str(amount) + " amount of " + ticker + ". Please check your data." + ENDC)

        if printDetails and len(table) > 0:
            print("")

        return costBasis

    def printDetails(self, printDetails, amount, ticker, row):
        if printDetails:
            sDate = row["date"].strftime(self.config.dateFormat())
            currentCost = round(row["price"] / row["amount"], 4)
            msg = str(row.doc_id) + ": " + sDate + " / " + row["type"] + " "  + str(row["amount"]) + " " + ticker + " for " + str(row["price"]) + "$ (" + str(currentCost) + "$)"
            if row["description"] is not None:
                msg += " (" + row["description"] + ")"

            print(msg)

        if amount == 0:
            print("----- No more " + ticker + " -----")
            print("")
