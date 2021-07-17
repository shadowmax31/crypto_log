from datetime import datetime

from cost_basis import CostBasis
from capital_gain import CapitalGain
from amount import Amount
from const import GREEN, YELLOW, ENDC, RED, Tables
from config import Config

class Report:
    def __init__(self, db):
        self.db = db
        self.config = Config()

    def costBasis(self, ticker, details):
        cost = CostBasis(self.db)
        costBasis = cost.calculate(ticker, None, details)
        realCost = round(costBasis * Amount(self.db).calculate(ticker), 2)

        if costBasis is not None:
            msg =  "Cost basis for " + ticker + ": " + YELLOW + str(costBasis) + "$" + ENDC + " (" + str(realCost) + "$)"
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
        amount = Amount(self.db)

        print("You have " + GREEN + str(amount.calculate(ticker)) + ENDC + " " + ticker)
            

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

