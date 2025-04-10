-- ==================================
-- ===== Pipeline Output Snapshot =====
-- ==================================

-- 1. Overall Counts
SELECT 'Total Customers' AS Metric, COUNT(*) AS Value FROM customers
UNION ALL
SELECT 'Total Products' AS Metric, COUNT(*) AS Value FROM products
UNION ALL
SELECT 'Total Orders' AS Metric, COUNT(*) AS Value FROM orders;

-- 2. Top Spenders (Display Top 5)
SELECT '--- Top 5 Spenders ---' AS Header; 
SELECT CustomerName, TotalSpent
FROM top_spenders
ORDER BY TotalSpent DESC
LIMIT 5;

-- 3. Top Products (Display Top 5)
SELECT '--- Top Selling Product(s) ---' AS Header;
SELECT ProductName, Count AS OrdersCount
FROM top_product
ORDER BY Count DESC;

-- 4. Highest Return Rates (Display Top 5)
SELECT '--- Top 5 Product Return Rates ---' AS Header; 
SELECT ProductName, printf('%.2f%%', ReturnRate * 100) AS ReturnRatePercentage
FROM product_return_rates
WHERE ReturnRate IS NOT NULL -- Avoid NULLs if a product had 0 orders
ORDER BY ReturnRate DESC
LIMIT 5;

-- 5. Monthly Order Volume 
SELECT '--- Monthly Order Volume ---' AS Header; 
SELECT strftime(Month, '%Y-%m') AS OrderMonth, OrderCount
FROM monthly_order_volume
ORDER BY Month DESC
LIMIT 6;

-- 6. Anomaly Summary
SELECT 'Total Anomalous Orders' AS Metric, COUNT(*) AS Separator FROM order_anomalies;

-- 7. Anomalous Customers (Display up to 5)
SELECT '--- Anomalous Customers ---' AS Header; 
SELECT CustomerName, AnomalyCount
FROM customer_anomalies
ORDER BY AnomalyCount DESC
LIMIT 5;

-- 8. Anomalous Orders (Display up to 5)
SELECT '--- Anomalous Orders ---' AS Header;  
SELECT Id, CustomerId, ProductId, Status, CancelReason -- Select key fields
FROM order_anomalies
LIMIT 5;

-- 9. Recent Orders (Display up to 5)
SELECT '--- Recent Orders ---' AS Header; 
SELECT Id, CustomerId, ProductId, Date, Total -- Select key fields
FROM orders
ORDER BY Date DESC
LIMIT 5;

-- ==================================
-- ========= End of Snapshot =========
-- ==================================
