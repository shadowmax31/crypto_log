from const import TransactionType, GREEN, ENDC, BOLD, YELLOW
from cost_basis import CostBasis
from config import Config
from util import listTickers

from operator import itemgetter
from tinydb import Query

class CapitalGain:

    def __init__(self, db):
        self.db = db
        self.config = Config()


    def gain(self, year, details):
        
        gain = 0
        for ticker in listTickers(self.db):
            def checkYear(val, year):
                return val.year == year

            q = Query()
            table = self.db.table(ticker).search(q.date.test(checkYear, year))

            for row in sorted(table, key=itemgetter("date")):
                tmp = None
                if(row["type"] == TransactionType.SELL.name):
                    tmp = self.calculateGain(ticker, row)
                    gain += tmp

                    if details:
                        sDate = row["date"].strftime(self.config.dateFormat())
                        msg = sDate + ": "
                        msg += GREEN + ticker + "_" + str(row.doc_id).zfill(4) + ENDC
                        msg += " - Capital Gain " + str(tmp) + "$"
                        msg += " (" + str(row["price"]) + "$)"
                        print(msg)

        return gain
                    

    def calculateGain(self, ticker, taxableEvent):
        cost = CostBasis(self.db)
        costBasis = cost.calculate(ticker, taxableEvent.doc_id, False)
        if costBasis is None:
            raise Exception(BOLD + YELLOW + "The capital gain cannot be calculated accurately" + ENDC)
            

        value = taxableEvent["price"] - (costBasis * taxableEvent["amount"])

        return round(value, 4)
