ALTER TABLE
    installments RENAME COLUMN step TO installment_number;

ALTER TABLE
    installments DROP COLUMN amount;

ALTER TABLE
    installments
ADD
    COLUMN value NUMERIC NOT NULL;

ALTER TABLE installments DROP COLUMN payment_date;
ALTER TABLE installments DROP COLUMN month_reference;
ALTER TABLE installments DROP COLUMN year_reference;
