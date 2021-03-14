import unittest
import sys

from tinydb import TinyDB
from tinydb.storages import MemoryStorage

from datetime import datetime, timedelta

sys.path.append("../src")
from transaction import Transaction
from cost_basis import CostBasis
from undo import Undo
from capital_gain import CapitalGain

class TestUndo(unittest.TestCase):

    def testUndo(self):
        date = datetime.now()

        db = TinyDB(storage=MemoryStorage)

        transaction = Transaction(db)
        date = self.initBuy(transaction, date)

        # Get the cost basis with basic transactions
        cost = CostBasis(db)
        value = cost.calculate("btc", False)
        
        # Undo the last transaction
        undo = Undo(db)
        undo.undo()

        valueAfterUndo = cost.calculate("btc", False)
        
        # Assert the cost basis after and before the undo
        self.assertTrue(value == 20000)
        self.assertTrue(valueAfterUndo == 10000)

        # Do an exchange for crypto (this includes a buy and sell)
        date = self.incDate(date)
        transaction.exchange(date, 0.5, "btc", 2, "eth", 1000, "Description")

        # Get the new cost basis for eth
        value = cost.calculate("eth", False)
        self.assertTrue(value, 1000)

        # Get the capital gain
        cg = self.returnCapitalGain(db)
        self.assertTrue(cg == -4000)

        # Undo the exchange
        undo.undo()

        # Get the new capital gain
        cg = self.returnCapitalGain(db)
        self.assertTrue(cg == 0)

        # There should not be any eth anymore 
        value = cost.calculate("eth")
        self.assertTrue(value is None)

        
    def initBuy(self, transaction, date):
        date = self.incDate(date)
        transaction.buy(date, 1, "btc", 10000, "Description")

        date = self.incDate(date)
        transaction.buy(date, 0.5, "btc", 20000, "Description")

        return date

    def returnCapitalGain(self, db):
        capitalGain = CapitalGain(db)
        gain = capitalGain.gain(datetime.now().year, False)

        return gain


    def incDate(self, date):
        return date + timedelta(minutes=1) 

if __name__ == '__main__':
    unittest.main()
