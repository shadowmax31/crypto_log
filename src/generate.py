import csv
from generate_algo import GenShakePay, GenCryptoDotCom, GenNewton

from config import Config

class Generate:

    def __init__(self, db, path):
        self.db = db
        self.path = path
        self.config = Config()


    def gen(self):
        with open(self.path, newline="") as csvfile:
            # Opens the CSV file
            csvFile = csv.reader(csvfile, delimiter=",")

            # next --> Skips the header (first row)
            algo = self.algoFactory(next(csvFile))

            notUsed = []
            print("#!/bin/bash")
            for row in csvFile:
                if not algo.gen(row):
                    notUsed.append(row)

            if len(notUsed) > 0:
                print("")
                algo.comment("List of transactions that were not used", None)

            for row in notUsed:
                algo.comment("", row)


    # Picks the right class to import the CSV
    def algoFactory(self, row):
        algo = None
        if row[0] == "Transaction Type" or row[1] == "Date" or row[2] == "Amount Debited" or row[3] == "Debit Currency":
            algo = GenShakePay(self.path, self.db)
        elif row[0] == "Timestamp (UTC)" or row[1] == "Transaction Description" or row[2] == "Currency" or row[3] == "Amount":
            algo = GenCryptoDotCom(self.path, self.db)
        elif row[0] == "Date" or row[1] == "Type" or row[2] == "Received Quantity" or row[3] == "Received Currency":
            algo = GenNewton(self.path, self.db)
        else:
            raise Exception("File not supported")

        return algo

