import csv
from generate_algo import GenShakePay, GenCryptoDotCom

from config import Config

class Generate:

    def __init__(self, db, path):
        self.db = db
        self.path = path
        self.config = Config()


    def gen(self):
        with open(self.path, newline="") as csvfile:
            csvFile = csv.reader(csvfile, delimiter=",")

            # Skips the header
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


    def algoFactory(self, row):
        algo = None
        if row[0] == "Transaction Type" or row[1] == "Date" or row[2] == "Amount Debited" or row[3] == "Debit Currency":
            algo = GenShakePay(self.path)
        elif row[0] == "Timestamp (UTC)" or row[1] == "Transaction Description" or row[2] == "Currency" or row[3] == "Amount":
            algo = GenCryptoDotCom(self.path)
        else:
            raise Exception("File not supported")

        return algo

