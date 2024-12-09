-- Create enum types
CREATE TYPE entry_type AS ENUM (
    'REVENUE',
    'EXPENSE'
);

CREATE TYPE entry_status AS ENUM (
    'PENDING',
    'CANCELED',
    'COMPLETED'
);

CREATE TYPE bank AS ENUM (
    'NUBANK',
    'INTER',
    'SANTANDER',
    'ITAU',
    'BRADESCO',
    'BANCO_DO_BRASIL',
    'SWILE'
);

-- INVOICES TABLE
CREATE TABLE invoices (
    invoice_id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    month INT NOT NULL CHECK (month BETWEEN 1 AND 12),
    year SMALLINT NOT NULL,
    is_parent BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX unique_parent_per_month_year
ON invoices (month, year)
WHERE is_parent = TRUE;

-- ACCOUNT TABLE
CREATE TABLE accounts (
    account_id UUID PRIMARY KEY,
    bank_name bank NOT NULL,
    owner TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- ENTRIES TABLE
CREATE TABLE entries (
    entry_id UUID PRIMARY KEY,
    invoice_id UUID NOT NULL REFERENCES invoices(invoice_id),
    entry_type entry_type NOT NULL,
    description TEXT NOT NULL,
    value NUMERIC NOT NULL,
    due_date DATE NOT NULL,
    tag TEXT NOT NULL, 
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    status entry_status NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- SUB INVOICES TABLE
CREATE TABLE invoice_relations (
    parent_invoice_id UUID NOT NULL,
    child_invoice_id UUID NOT NULL,
    PRIMARY KEY (parent_invoice_id, child_invoice_id),
    FOREIGN KEY (parent_invoice_id) REFERENCES invoices(invoice_id),
    FOREIGN KEY (child_invoice_id) REFERENCES invoices(invoice_id)
);
