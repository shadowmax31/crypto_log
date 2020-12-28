import sys
from tinydb import Query

class History:
    def __init__(self, db):
        cmd = " ".join(sys.argv)
        self.db = db
        self.db.table("history").insert({
            "command": cmd
            })

    def all(self):
        return self.db.table("history").all()

    def transactionOnly(self):
        q = Query()
        return self.db.table("history").search(
                q.command.search("buy|sell|exchange")
                )
