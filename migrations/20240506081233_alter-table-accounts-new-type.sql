ALTER TABLE
    accounts DROP COLUMN currency;

CREATE TYPE bank_name AS ENUM(
    'NUBANK',
    'INTER',
    'SANTANDER',
    'ITAÚ',
    'BRADESCO',
    'BANCO DO BRASIL'
);

ALTER TABLE
    accounts
ALTER COLUMN
    bank_name
SET
    NOT NULL,
ALTER COLUMN
    bank_name TYPE bank_name USING bank_name :: bank_name;