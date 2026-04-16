SELECT 
    received_loan, 
    (SUM(temporal_workers_total) + SUM(permanent_workers_total)) / SUM(total_area_mz) AS labor_to_land_ratio
FROM farms 
GROUP BY received_loan;