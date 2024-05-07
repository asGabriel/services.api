ALTER TABLE
    accounts
ALTER COLUMN
    account_type TYPE TEXT;

DROP TYPE account_type;

CREATE TYPE account_type AS ENUM('CREDIT', 'DEBIT', 'HYBRID');

ALTER TABLE
    accounts
ALTER COLUMN
    account_type TYPE account_type USING account_type :: account_type;