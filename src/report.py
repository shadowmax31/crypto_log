from datetime import datetime

from cost_basis import CostBasis
from capital_gain import CapitalGain
from const import GREEN, YELLOW, ENDC, RED, TransactionType, Tables
from config import Config

class Report:
    def __init__(self, db):
        self.db = db
        self.config = Config()

    def costBasis(self, ticker, details):
        cost = CostBasis(self.db)
        costBasis = cost.calculate(ticker, None, details)

        if costBasis is not None:
            msg =  "Cost basis for " + ticker + ": " + YELLOW + str(costBasis) + "$" + ENDC
        else:
            msg = "No " + ticker + " found"

        print(msg)


    def capitalGain(self, year, details):
        if year is None:
            year = datetime.now().year

        cg = CapitalGain(self.db)
        try:
            gain = cg.gain(year, details)
            if details and gain != 0:
                print("")
                        
            gain = round(gain, 2)
            if gain > 0:
                print("You have " + YELLOW + str(gain) + "$" + ENDC + " of capital gain")
            elif gain < 0:
                print("You have " + RED + str(abs(gain)) + "$" + ENDC + " of capital losses")
            else:
                print(GREEN + "You have no capital gains or losses" + ENDC)
        except Exception as e:
            print(e)




    def amount(self, ticker):
        table = self.db.table(ticker)

        amount = 0
        for row in table:
            if row["type"] == TransactionType.BUY.name:
                amount += row["amount"]
            elif row["type"] == TransactionType.SELL.name:
                amount -= row["amount"]
            else:
                raise Exception("Transaction type [" + row["type"] + "] not implemented") 

        print("You have " + GREEN + str(amount) + ENDC + " " + ticker)
            

    def history(self, history, all, withIds):
        if all:
            historyList = history.all()
        else:
            historyList = history.transactionOnly()

        for cmd in historyList:
            line = cmd["command"] 
            if withIds:
                line = str(cmd.doc_id) + ": " + line

            print(line)

