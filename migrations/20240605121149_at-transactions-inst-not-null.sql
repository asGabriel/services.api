UPDATE
    transactions
SET
    installment_number = 1;

ALTER TABLE
    transactions
ALTER COLUMN
    installment_number
SET
    NOT NULL;