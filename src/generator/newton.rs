// class GenNewton(AbstractGen):

//     def gen(self, row):
//         found = True
        
//         if row[1] == "TRADE":
//             date = self.convertDate(row[0], "%m/%d/%Y %H:%M:%S", None)
//             amount = row[2]
//             ticker = row[3]
//             price = row[4]
//             if price == "":
//                 price = "0"

//             if row[5] == "CAD":
//                 self.genTransactionString("buy", date, amount, ticker, price)
//             else:
//                 self.comment("This transaction type is not supported. Please add an issue on github.", row)
//         else:
//             found = False

//         return found
