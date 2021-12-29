import sys, subprocess
from tinydb import Query
from const import Tables
from util import listTickers

class History:
    def __init__(self, db):
        cmd = "crypto " + subprocess.list2cmdline(sys.argv[1:])
        self.db = db
        self.db.table(Tables.HISTORY.value).insert({
            "command": cmd
            })

    def all(self):
        return self.db.table(Tables.HISTORY.value).all()

    def transactionOnly(self):
        q = Query()
        return self.db.table(Tables.HISTORY.value).search(
                q.command.search("buy|sell|exchange|undo")
                )
    
    def from_source(self, config):
        tables = listTickers(self.db)

        for table in tables:
            db_table = self.db.table(table)

            print("# " + table)
            for row in db_table:
                t_type = row["type"].lower()
                date = row["date"].strftime(config.dateFormat())
                print("crypto " + t_type + " " + date + " " + str(row["amount"]) + " " + table + " " + str(row["price"]) + " \"" + row["description"] + "\"")
            
            print("")
                
            # "date": self.convertStrToDate(date),
            # "amount": Decimal(str(amount)),
            # "price": Decimal(str(price)),
            # "description": description,
            # "type": tType
        
        
