-- transaction table
CREATE TABLE Transactions (
    transaction_id UUID PRIMARY KEY,
    type TEXT NOT NULL,
    description TEXT,
    amount FLOAT NOT NULL,
    due_date DATE NOT NULL,
    category TEXT,
    account_id UUID NOT NULL,
    recurring BOOLEAN NOT NULL DEFAULT FALSE,
    recurrence_frequency TEXT,
    recurrence_duration_months INT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    note TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT NULL,
    deleted_at TIMESTAMPTZ DEFAULT NULL
);

-- performance indexes to transactions table
CREATE INDEX idx_type ON Transactions (type);

CREATE INDEX idx_account_id ON Transactions (account_id);

-- accounts table
CREATE TABLE Accounts (
    account_id UUID PRIMARY KEY,
    account_name TEXT NOT NULL,
    bank_name TEXT,
    currency VARCHAR(3) NOT NULL DEFAULT 'BRL',
    owner TEXT NOT NULL,
    note TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT NULL,
    deleted_at TIMESTAMPTZ DEFAULT NULL
);

ALTER TABLE
    Transactions
ADD
    CONSTRAINT fk_account_id FOREIGN KEY (account_id) REFERENCES Accounts(account_id) ON DELETE CASCADE;