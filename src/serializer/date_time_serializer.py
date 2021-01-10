from datetime import datetime
from tinydb_serialization import Serializer
from config import Config

class DateTimeSerializer(Serializer):
    OBJ_CLASS = datetime

    def __init__(self):
        self.config = Config()

    def encode(self, obj):
        return obj.strftime(self.config.dateFormat())

    def decode(self, s):
        return datetime.strptime(s, self.config.dateFormat())
    
