// class GenCryptoDotCom(AbstractGen):
    
//     def gen(self, row):
//         found = True
//         date = self.convertDate(row[0], "%Y-%m-%d %H:%M:%S")
//         amount = row[3]
//         ticker = row[2]
//         price = row[7]
//         if price == "":
//             price = "0"

//         toTicker = row[4]
//         toAmount = row[5]

//         if row[9] == "reimbursement" or row[9] == "referral_card_cashback" or row[9] == "crypto_earn_interest_paid":
//             self.genTransactionString("buy", date, amount, ticker, 0)
//         elif row[9] == "viban_purchase":
//             self.genTransactionString("buy", date, toAmount, toTicker, price)
//             self.comment("Check this transaction ^", None)
//         elif row[9] == "card_top_up" or row[9] == "crypto_transfer" or (row[9] == "crypto_viban_exchange" and toTicker == "CAD"):
//             self.genTransactionString("sell", date, abs(Decimal(amount)), ticker, abs(Decimal(price)))
//         elif row[9] == "crypto_exchange":
//             self.genTransactionExchangeString(date, abs(Decimal(amount)), ticker, toAmount, toTicker, abs(Decimal(price)))
//         elif row[9] == "dust_conversion_credited" or row[9] == "dust_conversion_debited":
//             if not self.transaction.transactionExists(date, ticker, True):
//                 self.echo("", row)
//                 self.echo(RED + "^ Please insert this row manually, this is not supported yet" + ENDC, None)
//                 self.echo(YELLOW + "Use this date: " + date + ENDC, None)
//                 self.echo("", None)
//             else:
//                 self.comment("The transaction exists", row)
//         else:
//             found = False

//         return found