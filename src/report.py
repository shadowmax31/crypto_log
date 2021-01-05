from datetime import datetime

from cost_basis import CostBasis
from capital_gain import CapitalGain
from const import GREEN, YELLOW, ENDC, RED, TransactionType
from tinydb import Query
from config import Config

class Report:
    def __init__(self, db):
        self.db = db
        self.config = Config()

    def costBasis(self, ticker, details):
        cost = CostBasis(self.db)
        costBasis = cost.calculate(ticker, details)

        if costBasis is not None:
            msg =  "Cost basis for " + ticker + ": " + YELLOW + str(costBasis) + "$" + ENDC
        else:
            msg = "No " + ticker + " found"

        print(msg)


    def capitalGain(self, year, details):
        if year is None:
            year = datetime.now().year

        def checkYear(val, year):
            transactionDate = datetime.strptime(val, self.config.dateFormat()).date()
            return transactionDate.year == year

        q = Query()
        table = self.db.table("capital_gain").search(q.date.test(checkYear, year))

        capitalGain = 0
        for row in table:
            cg = CapitalGain(self.db, row)
            tmp = cg.gain()
            capitalGain += tmp

            if details:
                msg = str(row.doc_id) + " (" + row["date"] + "): Capital Gain " + str(tmp) + "$"
                msg += " (" + GREEN + row["source_ticker"] + ENDC + " / " + GREEN + "Id: " + str(row["source_id"]) + ENDC + ")"
                print(msg)

        if details and len(table):
            print("")

        capitalGain = round(capitalGain, 2)
        if capitalGain > 0:
            print("You have " + YELLOW + str(capitalGain) + "$" + ENDC + " of capital gain")
        elif capitalGain < 0:
            print("You have " + RED + str(abs(capitalGain)) + "$" + ENDC + " of capital losses")
        else:
            print(GREEN + "You have no capital gains or losses" + ENDC)

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

