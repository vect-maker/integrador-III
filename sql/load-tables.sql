
BEGIN;

CREATE OR REPLACE VIEW farms AS
    SELECT * FROM read_parquet('data/farms.parquet');

COMMIT;
