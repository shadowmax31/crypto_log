from const import Tables

def listTickers(db):
    tables = db.tables()
    
    # Remove all system tables
    for table in Tables:
        if(table.value in tables):
            tables.remove(table.value)

    return tables
