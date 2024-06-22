CREATE TABLE IF NOT EXISTS reccurence_transactions (
    recurrence_transaction_id UUID PRIMARY KEY,
    account_id UUID NOT NULL,
    transaction_id UUID NOT NULL,
    description text NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    frequency recurrence_frequency,
    is_active boolean DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT NULL,
    deleted_at TIMESTAMPTZ DEFAULT NULL,
    CONSTRAINT fk_account_id FOREIGN KEY (account_id) REFERENCES accounts(account_id) ON DELETE CASCADE,
    CONSTRAINT fk_transaction_id FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS generated_transaction (
    id SERIAL PRIMARY KEY,
    recurrence_transaction_id UUID NOT NULL,
    transaction_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ DEFAULT NULL,
    CONSTRAINT fk_recurrence_transaction_id FOREIGN KEY (recurrence_transaction_id) REFERENCES reccurence_transactions(recurrence_transaction_id) ON DELETE CASCADE,
    CONSTRAINT fk_transaction_id FOREIGN KEY (transaction_id) REFERENCES transactions(transaction_id) ON DELETE CASCADE
);