import unittest
import sys
from tinydb import TinyDB
from tinydb.storages import MemoryStorage

sys.path.append("../src")
from transaction import Transaction
from cost_basis import CostBasis
from capital_gain import CapitalGain

class TestTransactions(unittest.TestCase):

    def testBuy(self):
        db = TinyDB(storage=MemoryStorage)

        transaction = Transaction(db)
        transaction.buy("2020-01-01T00:00:00", 1, "btc", 10000, "Description")

        # Validate the cost basis
        cost = CostBasis(db)
        value = cost.calculate("btc", False)
        self.assertEqual(value, 10000)

        transaction.buy("2020-01-01T00:00:00", 0.5, "btc", 20000, "Description")

        # Validate the cost basis
        value = cost.calculate("btc", False)
        self.assertEqual(value, 20000)

        # There should be no value in the capital_gain table
        numTaxEvents = len(db.table("capital_gain"))
        self.assertEqual(numTaxEvents, 0)

    def testBuySell(self):
        db = TinyDB(storage=MemoryStorage)

        transaction = Transaction(db)
        self.initBuy(transaction)

        cost = CostBasis(db)
        initialCostBasis = cost.calculate("btc", False)

        # Validate cost basis
        costEth = cost.calculate("eth", False)
        self.assertEqual(costEth, 2500)

        transaction.sell("2020-01-05T00:00:00", 0.25, "btc", 12000, "Description")

        # When you sell, the cost basis should not change
        newCostBasis = cost.calculate("btc", False)
        self.assertEqual(initialCostBasis, newCostBasis)
        self.assertEqual(newCostBasis, 20000)
        
        # Test the capital gain
        gain = self.returnCapitalGain(db)
        self.assertEqual(gain, 7000)

        # Test buy after sell
        transaction.buy("2020-01-10T00:00:00", 0.30, "btc", 15000, "Description")
        newCostBasis = cost.calculate("btc", False)
        self.assertEqual(newCostBasis, 25806.4516)

        # Second sell test to check the capital gain with multiple tax events
        transaction.sell("2020-01-11T00:00:00", 0.52, "btc", 1000, "Description")
        newCostBasis = cost.calculate("btc", False)
        self.assertEqual(newCostBasis, 25806.4516)

        # Check the capital gain with multiple tax events / sell
        gain = self.returnCapitalGain(db)
        self.assertEqual(gain, -5419.3547)

    def testExchange(self):
        db = TinyDB(storage=MemoryStorage)
        cost = CostBasis(db)

        transaction = Transaction(db)
        self.initBuy(transaction)

        transaction.exchange("2020-01-12T00:00:00", 0.25, "btc", 1, "eth", 1000, "Description")

        # Validate the different cost basis after an exchange of crypto
        costBtc = cost.calculate("btc", False)
        costEth = cost.calculate("eth", False)
        self.assertEqual(costBtc, 20000)
        self.assertEqual(costEth, 2000)
        
        gain = self.returnCapitalGain(db)

        # Validate the capital gain
        self.assertEqual(gain, -4000)


    def initBuy(self, transaction):
        transaction.buy("2020-01-01T00:00:00", 1, "btc", 10000, "Description")
        transaction.buy("2020-01-01T00:00:00", 0.5, "btc", 20000, "Description")

        transaction.buy("2020-01-01T00:00:00", 2, "eth", 5000, "Description")


    def returnCapitalGain(self, db):
        table = db.table("capital_gain")
        gain = 0
        for row in table:
            capitalGain = CapitalGain(db, row)
            gain += capitalGain.gain()

        return gain


if __name__ == '__main__':
    unittest.main()

