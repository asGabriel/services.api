DELETE FROM invoices;

ALTER TABLE invoices
    ADD CONSTRAINT unique_month_year UNIQUE (month, year);