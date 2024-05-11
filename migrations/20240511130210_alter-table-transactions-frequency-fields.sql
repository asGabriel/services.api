ALTER TABLE transactions ALTER COLUMN amount TYPE DECIMAL(10,2);

ALTER TABLE transactions ADD COLUMN installment_number SMALLINT;

ALTER TABLE transactions ALTER COLUMN movement_type SET NOT NULL;

ALTER TABLE transactions ALTER COLUMN year_reference TYPE SMALLINT;

CREATE TABLE IF NOT EXISTS installments(
    installment_id UUID PRIMARY KEY,
    transaction_id UUID NOT NULL,
    step SMALLINT NOT NULL DEFAULT 1,
    due_date DATE NOT NULL,
    amount DECIMAL(10,2) NOT NULL,
	status status NOT NULL,
    payment_date DATE,
    month_reference month_reference NOT NULL,
    year_reference SMALLINT NOT NULL,
    CONSTRAINT fk_transaction_id FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id)
);
