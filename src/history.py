import sys, subprocess
from tinydb import Query

class History:
    def __init__(self, db):
        cmd = "crypto " + subprocess.list2cmdline(sys.argv[1:])
        self.db = db
        self.db.table("history").insert({
            "command": cmd
            })

    def all(self):
        return self.db.table("history").all()

    def transactionOnly(self):
        q = Query()
        return self.db.table("history").search(
                q.command.search("buy|sell|exchange|undo")
                )
