from datetime import datetime, timezone
from decimal import Decimal
from transaction import Transaction

from config import Config

class AbstractGen:


    def __init__(self, path, db):
        self.config = Config()
        self.path = path
        self.transaction = Transaction(db)


    def gen(self, row):
        pass


    def genTransactionString(self, transactionType, date, amount, ticker, price):
        msg = ""
        if self.transaction.transactionExists(date, ticker, True):
            msg = "# The transaction exists -- "

        print(msg + "crypto " + transactionType + " \"" + date + "\" " + str(amount) + " " + ticker + " " + str(price) + " \"" + self.path + "\"")

    
    def genTransactionExchangeString(self, date, amount, ticker, toAmount, toTicker, forPrice):
        msg = ""
        if self.transaction.transactionExists(date, ticker, True):
            msg = "# The transaction exists -- "

        print(msg + "crypto exchange" + " \"" + date + "\" " + str(amount) + " " + ticker + " " + 
                str(toAmount) + " " + toTicker + " " + str(forPrice) + " \"" + self.path + "\"")


    def comment(self, msg, row):
        if row is not None:
            if msg != "":
                msg += ": "
            msg += str(row)

        print("# " + msg)


    def convertDate(self, sDate, sFormat, pTimezone=timezone.utc):
        date = datetime.strptime(sDate, sFormat).replace(tzinfo=pTimezone).astimezone(tz=None)
        return date.strftime(self.config.dateFormat())


class GenNewton(AbstractGen):

    def gen(self, row):
        found = True
        
        if row[1] == "TRADE":
            date = self.convertDate(row[0], "%m/%d/%Y %H:%M:%S", None)
            amount = row[4]
            ticker = row[5]
            price = row[2]
            if price == "":
                price = "0"

            if row[3] == "CAD":
                self.genTransactionString("buy", date, amount, ticker, price)
            else:
                self.comment("This transaction type is not supported. Please add an issue on github.", row)
        else:
            found = False

        return found


class GenCryptoDotCom(AbstractGen):
    
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
        elif row[9] == "card_top_up" or row[9] == "crypto_transfer":
            self.genTransactionString("sell", date, abs(Decimal(amount)), ticker, abs(Decimal(price)))
        elif row[9] == "crypto_exchange":
            toAmount = row[5]
            toTicker = row[4]

            self.genTransactionExchangeString(date, abs(Decimal(amount)), ticker, toAmount, toTicker, abs(Decimal(price)))
        else:
            found = False

        return found


class GenShakePay(AbstractGen):

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
 
