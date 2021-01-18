import csv
from datetime import datetime, timezone

from config import Config

class Generate:

    def __init__(self, db, path):
        self.db = db
        self.path = path
        self.config = Config()


    def fromShakepay(self):
        # Dates in the CSV are in UTC time (we will convert them to local time)
        with open(self.path, newline="") as csvfile:
            csvFile = csv.reader(csvfile, delimiter=",", quotechar="\"")

            notUsed = []

            first = True
            print("#!/bin/bash")
            for row in csvFile:
                if not first:
                    date = self.convertDate(row[1], "%Y-%m-%dT%H:%M:%S+00")
                    amount = row[4]
                    ticker = row[5]
                    price = row[2]
                    if price == "":
                        price = "0"

                    if row[0] == "purchase/sale":
                        if row[3] == "CAD":
                            self.genTransactionString("buy", date, amount, ticker, price)
                        else:
                            self.comment("This transaction type is not supported. Please add an issue on github.", row)
                    elif row[0] == "peer transfer":
                        self.genTransactionString("buy", date, amount, ticker, price)
                    else:
                        notUsed.append(row)
                else:
                    first = False

            if len(notUsed) > 0:
                print("")
                self.comment("List of transactions that were not used", None)

            for row in notUsed:
                self.comment("", row)


    def genTransactionString(self, transactionType, date, amount, ticker, price):
        print("crypto " + transactionType + " \"" + date + "\" " + amount + " " + ticker + " " + price + " \"" + self.path + "\"")

    def comment(self, msg, row):
        if row is not None:
            if msg != "":
                msg += ": "
            msg += str(row)

        print("# " + msg)


    def convertDate(self, sDate, sFormat):
        date = datetime.strptime(sDate, sFormat).replace(tzinfo=timezone.utc).astimezone(tz=None)
        return date.strftime(self.config.dateFormat())
        
