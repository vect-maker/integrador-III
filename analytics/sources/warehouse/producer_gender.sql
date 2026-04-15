SELECT producer_gender, COUNT(*) AS count FROM farms GROUP BY producer_gender ORDER BY count DESC;
