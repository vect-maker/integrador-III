
BEGIN;

CREATE OR REPLACE VIEW farms AS
    SELECT * FROM read_parquet('data/farms.parquet');

CREATE OR REPLACE VIEW parcels AS
    SELECT * FROM read_parquet('data/parcels.parquet');

COMMIT;
