import sys, subprocess
from tinydb import Query
from const import Tables

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
