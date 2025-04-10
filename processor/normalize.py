import duckdb as ddb

with ddb.connect("orders.duckdb") as con:
    con.execute(
        """
        CREATE TABLE IF NOT EXISTS customers (
            Id BIGINT,
            Name VARCHAR,
            Email VARCHAR,
            Address VARCHAR,
            Status VARCHAR
        )
    """
    )

    con.execute(
        """
        CREATE TABLE IF NOT EXISTS products (
            Id BIGINT,
            Name VARCHAR,
            Category VARCHAR,
            Price DOUBLE
        )
    """
    )

    con.execute(
        """
        CREATE TABLE IF NOT EXISTS orders (
            Id VARCHAR,
            CustomerId BIGINT,
            ProductId BIGINT,
            Date TIMESTAMP,
            Payment VARCHAR,
            Status VARCHAR,
            Discount DOUBLE,
            Quantity INTEGER,
            Total DOUBLE
        )
    """
    )

    con.execute(
        """
        INSERT INTO customers
        SELECT DISTINCT
            Customer.Id AS Id,
            Customer.Name AS Name,
            Customer.Email AS Email,
            Customer.Address AS Address,
            Customer.Status AS Status
        FROM incoming_orders
        WHERE Customer.Id NOT IN (SELECT Id FROM customers);
    """
    )

    con.execute(
        """
        INSERT INTO products
        SELECT DISTINCT
            Product.Id AS Id,
            Product.Name AS Name,
            Product.Category AS Category,
            Product.Price AS Price
        FROM incoming_orders
        WHERE Product.Id NOT IN (SELECT Id FROM products);
    """
    )

    con.execute(
        """
        INSERT INTO orders
        SELECT
            o.Id,
            o.Customer.Id AS CustomerId,
            o.Product.Id AS ProductId,
            o.Date,
            o.Payment,
            o.Status,
            o.Discount,
            o.Quantity,
            o.Total
        FROM incoming_orders o;
    """
    )

    con.execute("DROP TABLE IF EXISTS incoming_orders")
