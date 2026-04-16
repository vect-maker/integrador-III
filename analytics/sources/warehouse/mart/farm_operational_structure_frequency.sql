WITH raw_counts AS (
    SELECT 
        farm_operational_structure AS structure, 
        COUNT(*) AS frequency
    FROM farms
    GROUP BY 1
),
totals AS (
    SELECT 
        structure,
        frequency,
        SUM(frequency) OVER() as total_n,
        SUM(CASE WHEN structure IS NOT NULL THEN frequency ELSE 0 END) OVER() as valid_n
    FROM raw_counts
)
SELECT 
    upper(structure[1]) || lower(structure[2:]) AS "Operational Structure",
    frequency AS "Frequency",
    ROUND(100.0 * frequency / total_n, 1) AS "Percent",
    CASE 
        WHEN structure IS NULL THEN NULL 
        ELSE ROUND(100.0 * frequency / valid_n, 1) 
    END AS "Valid Percent",
    ROUND(SUM(100.0 * frequency / valid_n) OVER (ORDER BY frequency DESC), 1) AS "Cumulative Percent"
FROM totals
ORDER BY frequency DESC;