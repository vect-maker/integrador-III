BEGIN;

DROP VIEW IF EXISTS farms;
DROP VIEW IF EXISTS parcels;

CREATE VIEW farms AS SELECT * FROM read_parquet('data/farms-01.parquet');
CREATE VIEW parcels AS SELECT * FROM read_parquet('data/parcels-01.parquet');

COMMIT;
