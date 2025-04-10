import pandas as pd
import duckdb as ddb
import numpy as np


def save_customer_anomalies(con):
    con.execute("DROP TABLE IF EXISTS customer_anomalies")
    con.execute(
        """
        CREATE TABLE IF NOT EXISTS customer_anomalies AS
        SELECT
            o.CustomerId,
            c.Name AS CustomerName,
            COUNT(*) AS AnomalyCount
        FROM order_anomalies o
        JOIN customers c ON o.CustomerId = c.Id
        WHERE o.CancelReason <> 'BadCard'
        GROUP BY o.CustomerId, c.Name
        ORDER BY AnomalyCount DESC;
        """
    )


def save_product_return_rates(con):
    con.execute("DROP TABLE IF EXISTS product_return_rates")
    con.execute(
        """
        CREATE TABLE IF NOT EXISTS product_return_rates AS
        SELECT
            o.ProductId,
            p.Name AS ProductName,
            COUNT(*) AS TotalOrders,
            COUNT(*) FILTER (WHERE o.Status IN ('Cancelled', 'Refunded')) AS ReturnedOrders,
            1.0 * COUNT(*) FILTER (WHERE o.Status IN ('Cancelled', 'Refunded')) / COUNT(*) AS ReturnRate
        FROM orders o
        JOIN products p ON o.ProductId = p.Id
        GROUP BY o.ProductId, p.Name;
        """
    )


def save_monthly_order_volume(con):
    con.execute("DROP TABLE IF EXISTS monthly_order_volume")
    con.execute(
        """
        CREATE TABLE monthly_order_volume AS
        SELECT
            DATE_TRUNC('month', CAST(Date AS TIMESTAMP)) AS Month,
            COUNT(*) AS OrderCount
        FROM orders
        WHERE Status NOT IN ('Cancelled', 'Refunded')
        GROUP BY 1;
        """
    )


def save_top_customers(con):
    con.execute("DROP TABLE IF EXISTS top_spenders")
    con.execute(
        """
        CREATE TABLE IF NOT EXISTS top_spenders AS
        SELECT
            o.CustomerId,
            c.Name AS CustomerName,
            SUM(o.Total) AS TotalSpent
        FROM orders o
        JOIN customers c ON o.CustomerId = c.Id
        WHERE o.Status NOT IN ('Cancelled', 'Refunded')
        GROUP BY o.CustomerId, c.Name
        ORDER BY TotalSpent DESC
        LIMIT 10;
        """
    )


def save_anomalies(con):
    con.execute("DROP TABLE IF EXISTS order_anomalies")
    con.execute(
        """
        CREATE TABLE IF NOT EXISTS order_anomalies AS
        SELECT
            *,
            CASE 
                WHEN LENGTH(Id) != 36 THEN 'BadId'
                ELSE 'BadCard'
            END AS CancelReason
        FROM orders
        WHERE Status = 'Cancelled';
        """
    )


def save_most_popular_product(con):
    con.execute("DROP TABLE IF EXISTS top_product")
    con.execute(
        """
        CREATE TABLE IF NOT EXISTS top_product AS
        WITH product_counts AS (
            SELECT
                o.ProductId,
                p.Name AS ProductName,
                COUNT(*) AS Count
            FROM orders o
            JOIN products p ON o.ProductId = p.Id
            WHERE o.Status NOT IN ('Cancelled', 'Refunded')
            GROUP BY o.ProductId, p.Name
        ),
        max_count AS (
            SELECT MAX(Count) AS MaxCount FROM product_counts
        )
        SELECT pc.*
        FROM product_counts pc
        JOIN max_count mc ON pc.Count = mc.MaxCount;
        """
    )


with ddb.connect("orders.duckdb") as con:
    print("Saving anomalies...")
    save_anomalies(con)
    print("Saving most popular products...")
    save_most_popular_product(con)
    print("Saving monthly order volume...")
    save_monthly_order_volume(con)
    print("Saving number of customer anomalies...")
    save_customer_anomalies(con)
    print("Saving product return rates...")
    save_product_return_rates(con)
    print("Saving top spending customers...")
    save_top_customers(con)
    print("Done! Your data is ready to view in 'orders.duckdb'!")
