SELECT 
    department_id, 
    municipality_id, 
    farm_operational_structure,
    principal_activity, 
    total_area_mz, 
    received_loan 
FROM warehouse.farms 
LIMIT 10