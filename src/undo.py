from const import Tables

class Undo:
    INSERT = "insert"
    SKIP = "skip"

    def __init__(self, db):
        self.db = db


    def save(self, tableName, changeType, docId, numToUndo=1):
        # Saves the current command
        table = self.db.table(Tables.UNDO.value)

        table.insert({
            "table": tableName,
            "id": docId,
            "change_type": changeType,
            "num_to_undo": numToUndo
            })

    def undo(self):
        # Undo the last command
        undoTable = self.db.table(Tables.UNDO.value)

        maxId = 0
        for row in undoTable:
            if row.doc_id > maxId:
                maxId = row.doc_id

        if maxId > 0:
            self.undoLine(undoTable, maxId)


    def undoLine(self, undoTable, maxId):
        undoInfo = undoTable.get(doc_id=maxId)
        undoTable.remove(doc_ids=[maxId])
                    
        if undoInfo["change_type"] == self.INSERT:
            tbl = self.db.table(undoInfo["table"])
            tbl.remove(doc_ids=[undoInfo["id"]])
        elif undoInfo["change_type"] == self.SKIP:
            pass

        # Some commands (like exchange) have to undo multiple changes in the database
        for i in range(undoInfo["num_to_undo"] - 1):
            self.undo()
