from datetime import datetime, timezone
from decimal import Decimal
from transaction import Transaction
from const import RED, ENDC, YELLOW

from config import Config

class AbstractGen:


    def __init__(self, path, db):
        self.config = Config()
        self.path = path
        self.transaction = Transaction(db)


    def gen(self, row):
        pass


    # Generate the transaction (buy or sell) command
    def genTransactionString(self, transactionType, date, amount, ticker, price):
        msg = ""
        if self.transaction.transactionExists(date, ticker, True):
            msg = "# The transaction exists -- "

        print(msg + "crypto " + transactionType + " \"" + date + "\" " + str(amount) + " " + ticker + " " + str(price) + " \"" + self.path + "\"")
 
    # Generate the exchange transaction command
    def genTransactionExchangeString(self, date, amount, ticker, toAmount, toTicker, forPrice):
        msg = ""
        if self.transaction.transactionExists(date, ticker, True):
            msg = "# The transaction exists -- "

        print(msg + "crypto exchange" + " \"" + date + "\" " + str(amount) + " " + ticker + " " + 
                str(toAmount) + " " + toTicker + " " + str(forPrice) + " \"" + self.path + "\"")


    def echo(self, msg, row):
        msg = self.addRowToMsg(msg, row)

        print("echo \"" + msg + "\"")

    def comment(self, msg, row):
        msg = self.addRowToMsg(msg, row)

        print("# " + msg)

    def addRowToMsg(self, msg, row):
        if row is not None:
            if msg != "":
                msg += ": "
            msg += str(row)

        return msg


    def convertDate(self, sDate, sFormat, pTimezone=timezone.utc):
        date = datetime.strptime(sDate, sFormat).replace(tzinfo=pTimezone).astimezone(tz=None)
        return date.strftime(self.config.dateFormat())


class GenNewton(AbstractGen):

    def gen(self, row):
        found = True
        
        if row[1] == "TRADE":
            date = self.convertDate(row[0], "%m/%d/%Y %H:%M:%S", None)
            amount = row[2]
            ticker = row[3]
            price = row[4]
            if price == "":
                price = "0"

            if row[5] == "CAD":
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

        toTicker = row[4]
        toAmount = row[5]

        if row[9] == "reimbursement" or row[9] == "referral_card_cashback" or row[9] == "crypto_earn_interest_paid":
            self.genTransactionString("buy", date, amount, ticker, 0)
        elif row[9] == "viban_purchase":
            self.genTransactionString("buy", date, toAmount, toTicker, price)
            self.comment("Check this transaction ^", None)
        elif row[9] == "card_top_up" or row[9] == "crypto_transfer":
            self.genTransactionString("sell", date, abs(Decimal(amount)), ticker, abs(Decimal(price)))
        elif row[9] == "crypto_exchange":
            self.genTransactionExchangeString(date, abs(Decimal(amount)), ticker, toAmount, toTicker, abs(Decimal(price)))
        elif row[9] == "dust_conversion_credited" or row[9] == "dust_conversion_debited":
            if not self.transaction.transactionExists(date, ticker, True):
                self.echo("", row)
                self.echo(RED + "^ Please insert this row manually, this is not supported yet" + ENDC, None)
                self.echo(YELLOW + "Use this date: " + date + ENDC, None)
                self.echo("", None)
            else:
                self.comment("The transaction exists", row)
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
 
