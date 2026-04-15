BEGIN;


CREATE OR REPLACE TABLE farms AS 
    SELECT * FROM read_parquet('data/farms.parquet');

CREATE OR REPLACE TABLE parcels AS 
    SELECT * FROM read_parquet('data/parcels.parquet');

COMMIT;
