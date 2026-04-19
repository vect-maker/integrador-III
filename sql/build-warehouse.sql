BEGIN;

CREATE OR REPLACE TABLE farms AS 
    SELECT * FROM read_parquet('data/farms.parquet');

COMMIT;
