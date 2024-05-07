ALTER TABLE
    accounts DROP COLUMN name;

CREATE TYPE account_type AS ENUM('CREDIT', 'DEBIT', 'HYDRID');

ALTER TABLE
    accounts
ADD
    COLUMN account_type account_type;