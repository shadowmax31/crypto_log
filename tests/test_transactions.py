import unittest
import sys
from tinydb import TinyDB
from tinydb.storages import MemoryStorage

from datetime import datetime, timedelta

sys.path.append("../src")
from transaction import Transaction
from cost_basis import CostBasis
from capital_gain import CapitalGain

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

        # Test the transaction ordering by date
        later = date + timedelta(hours=2)
        transaction.buy(later, 1, "btc", 1000, "Description")
        costWithLaterTransaction = cost.calculate("btc")
        self.assertEqual(costWithLaterTransaction, 12400)

        date = self.incDate(date)
        sellId = transaction.sell(date, 0.25, "btc", 12000, "Description")

        # When you sell, the cost basis should not change
        newCostBasis = cost.calculate("btc", sellId)
        self.assertEqual(initialCostBasis, newCostBasis)
        self.assertEqual(newCostBasis, 20000)
        
        # Test the capital gain
        gain = self.returnCapitalGain(db)
        self.assertEqual(gain, 7000)

        # Test buy after sell
        date = self.incDate(date)
        docId = transaction.buy(date, 0.30, "btc", 15000, "Description")
        newCostBasis = cost.calculate("btc", docId)
        self.assertEqual(newCostBasis, 25806.4516)

        # Second sell test to check the capital gain with multiple tax events
        date = self.incDate(date)
        sellId = transaction.sell(date, 0.52, "btc", 1000, "Description")
        newCostBasis = cost.calculate("btc", sellId)
        self.assertEqual(newCostBasis, 25806.4516)

        # Check the capital gain with multiple tax events / sell
        gain = self.returnCapitalGain(db)
        self.assertEqual(gain, -5419.3547)

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

