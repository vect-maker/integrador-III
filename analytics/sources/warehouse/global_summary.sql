SELECT 
    COUNT(*) as total_farms,
    SUM(total_area_mz) as total_area_mz,
    SUM(CASE WHEN received_loan = true THEN 1 ELSE 0 END) as financed_farms
FROM warehouse.farms