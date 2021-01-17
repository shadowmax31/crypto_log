from decimal import Decimal
from tinydb_serialization import Serializer
from config import Config

class DecimalSerializer(Serializer):
    OBJ_CLASS = Decimal

    def __init__(self):
        self.config = Config()

    def encode(self, obj):
        return str(obj)

    def decode(self, s):
        return Decimal(s)
    
