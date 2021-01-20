from datetime import datetime, timezone
from decimal import Decimal

from config import Config

class AbstactGen:


    def __init__(self, path):
        self.config = Config()
        self.path = path

    def gen(self, row):
        pass

    def genTransactionString(self, transactionType, date, amount, ticker, price):
        print("crypto " + transactionType + " \"" + date + "\" " + str(amount) + " " + ticker + " " + str(price) + " \"" + self.path + "\"")

    def comment(self, msg, row):
        if row is not None:
            if msg != "":
                msg += ": "
            msg += str(row)

        print("# " + msg)


    def convertDate(self, sDate, sFormat):
        date = datetime.strptime(sDate, sFormat).replace(tzinfo=timezone.utc).astimezone(tz=None)
        return date.strftime(self.config.dateFormat())


class GenCryptoDotCom(AbstactGen):
    
    def gen(self, row):
        found = True
        date = self.convertDate(row[0], "%Y-%m-%d %H:%M:%S")
        amount = row[3]
        ticker = row[2]
        price = row[7]
        if price == "":
            price = "0"

        if row[9] == "reimbursement" or row[9] == "referral_card_cashback":
            self.genTransactionString("buy", date, amount, ticker, 0)
        elif row[9] == "card_top_up":
            self.genTransactionString("sell", date, abs(Decimal(amount)), ticker, abs(Decimal(price)))
        else:
            found = False

        return found


class GenShakePay(AbstactGen):

    def gen(self, row):
        found = True

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
            found = False

        return found
 
