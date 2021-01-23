import unittest
import sys
from tinydb import TinyDB
from tinydb.storages import MemoryStorage

from datetime import datetime, timedelta

sys.path.append("../src")
from transaction import Transaction
from cost_basis import CostBasis
from capital_gain import CapitalGain

# This unittest will fail if you it run two hours before new year. 
# Just go to bed
class TestTransactions(unittest.TestCase):

    def testBuy(self):
        date = datetime.now()
        db = TinyDB(storage=MemoryStorage)

        transaction = Transaction(db)
        transaction.buy(date, 1, "btc", 10000, "Description")

        # Validate the cost basis
        cost = CostBasis(db)
        value = cost.calculate("btc")
        self.assertEqual(value, 10000)

        date = self.incDate(date)
        transaction.buy(date, 0.5, "btc", 20000, "Description")

        # Validate the cost basis
        value = cost.calculate("btc")
        self.assertEqual(value, 20000)

        # Test add the same date twice
        transaction.buy(date, 0.2, "btc", 20000, "Description")
        value = cost.calculate("btc")
        # The cost basis should not have changed
        self.assertEqual(value, 20000)

    def testBuySell(self):
        date = datetime.now()
        db = TinyDB(storage=MemoryStorage)

        transaction = Transaction(db)
        date = self.initBuy(transaction, date)

        cost = CostBasis(db)
        initialCostBasis = cost.calculate("btc")

        # Validate cost basis
        costEth = cost.calculate("eth")
        self.assertEqual(costEth, 2500)


        # Test the transaction ordering by date (adds a transaction at the end of the list)
        later = date + timedelta(hours=2)
        transaction.buy(later, 1, "btc", 1000, "Description")
        costWithLaterTransaction = cost.calculate("btc")
        self.assertEqual(costWithLaterTransaction, 12400)


        # When you sell, the cost basis should not change
        date = self.incDate(date)
        sellId = transaction.sell(date, 0.25, "btc", 12000, "Description")

        newCostBasis = cost.calculate("btc", sellId) # The transaction is not the last one
        self.assertEqual(initialCostBasis, newCostBasis)
        self.assertEqual(newCostBasis, 20000)
        # END - Sell test
        


        # Test the capital gain (with ordering by date). The buy transaction in 2 hours is ignored
        # Because the taxable event (sell) happened before
        gain = self.returnCapitalGain(db)
        self.assertEqual(gain, 7000)


        # Test sell the same date twice
        transaction.sell(date, 0.11, "btc", 12000, "Description")
        gain = self.returnCapitalGain(db)
        # The capital gain should not have changed
        self.assertEqual(gain, 7000)


        # Test earlier buy
        earlier = date - timedelta(hours=1)
        transaction.buy(earlier, 1, "btc", 1000, "Description")

        gain = self.returnCapitalGain(db)
        self.assertEqual(gain, 8900)
        # END - Test earlier buy

        

        # Test buy after sell
        date = self.incDate(date)
        docId = transaction.buy(date, 0.30, "btc", 15000, "Description")
        newCostBasis = cost.calculate("btc", docId) # The transaction is not the last one
        self.assertEqual(float(newCostBasis), 16823.5294)


        # Second sell test to check the capital gain with multiple tax events
        date = self.incDate(date)
        sellId = transaction.sell(date, 0.52, "btc", 1000, "Description")
        newCostBasis = cost.calculate("btc", sellId) # The transaction is not the last one
        self.assertEqual(float(newCostBasis), 16823.5294)


        # Check the capital gain with multiple tax events / sell
        gain = self.returnCapitalGain(db)
        self.assertEqual(float(gain), 1151.7647)


        # Check the value of cost basis (includes the later/last transaction)
        laterCost = cost.calculate("btc")
        self.assertEqual(float(laterCost), 11601.2425)


    def testExchange(self):
        date = datetime.now()

        db = TinyDB(storage=MemoryStorage)
        cost = CostBasis(db)

        transaction = Transaction(db)
        date = self.initBuy(transaction, date)

        date = self.incDate(date)
        transaction.exchange(date, 0.25, "btc", 1, "eth", 1000, "Description")

        # Validate the different cost basis after an exchange of crypto
        costBtc = cost.calculate("btc")
        costEth = cost.calculate("eth")
        self.assertEqual(costBtc, 20000)
        self.assertEqual(costEth, 2000)
        
        gain = self.returnCapitalGain(db)

        # Validate the capital gain
        self.assertEqual(gain, -4000)


    def initBuy(self, transaction, date):
        date = self.incDate(date)
        transaction.buy(date, 1, "btc", 10000, "Description")

        date = self.incDate(date)
        transaction.buy(date, 0.5, "btc", 20000, "Description")

        date = self.incDate(date)
        transaction.buy(date, 2, "eth", 5000, "Description")

        return date

    def incDate(self, date):
        return date + timedelta(minutes=1) 

    def returnCapitalGain(self, db):
        capitalGain = CapitalGain(db)
        gain = capitalGain.gain(datetime.now().year, False)

        return gain


if __name__ == '__main__':
    unittest.main()

