ALTER TABLE
    transactions DROP COLUMN installment_number;

ALTER TABLE
    transactions DROP COLUMN amount;

ALTER TABLE
    transactions
ADD
    COLUMN value NUMERIC NOT NULL;

ALTER TABLE
    transactions
ALTER COLUMN
    description DROP DEFAULT;