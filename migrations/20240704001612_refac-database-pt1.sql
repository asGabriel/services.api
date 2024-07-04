CREATE TABLE IF NOT EXISTS settlements (
    settlement_id uuid PRIMARY KEY,
    transaction_id uuid NOT NULL,
    installment_id uuid,
    paid_date date NOT NULL,
    paid_value numeric NOT NULL,
    discount numeric DEFAULT 0,
    fees numeric DEFAULT 0,
    attachment bytea,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT NULL,
    deleted_at TIMESTAMPTZ DEFAULT NULL,
    CONSTRAINT fk_installment FOREIGN KEY (installment_id) REFERENCES installments (installment_id),
    CONSTRAINT fk_transaction FOREIGN KEY (transaction_id) REFERENCES transactions (transaction_id)
);