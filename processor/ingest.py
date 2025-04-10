import os
import duckdb as ddb


def create_temp_table():
    with ddb.connect("orders.duckdb") as con:
        con.execute("CREATE TABLE incoming_orders AS (SELECT * FROM 'data.parquet')")


if __name__ == "__main__":
    if os.path.exists("data.parquet"):
        create_temp_table()
