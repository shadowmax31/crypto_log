from enum import Enum

# Colors
GREEN = '\033[92m'
YELLOW = '\033[93m'
RED = '\033[91m'
ENDC = '\033[0m'

# Name of the tables
class Tables(Enum):
    UNDO = "undo"
    CAPITAL_GAIN = "capital_gain"
    HISTORY = "history"

class TransactionType(Enum):
    BUY = "buy"
    SELL = "sell"

