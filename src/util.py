from const import Tables

# Simple function to list the tickers
def listTickers(db):
    tables = db.tables()
    
    # Remove all system tables from the list
    for table in Tables:
        if(table.value in tables):
            tables.remove(table.value)

    return tables
