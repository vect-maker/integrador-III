
BEGIN;

CREATE OR REPLACE VIEW farms AS
    SELECT * FROM read_parquet('data/farms.parquet');

CREATE OR REPLACE VIEW farms_raw AS
    SELECT * FROM read_parquet('data/farms_raw.parquet');

CREATE OR REPLACE VIEW parcels_raw AS
    SELECT * FROM read_parquet('data/parcels_raw.parquet');



COMMIT;
