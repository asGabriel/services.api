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

-- Create table invoices
CREATE TABLE invoices (
    invoice_id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    month INT NOT NULL CHECK (month BETWEEN 1 AND 12),
    year SMALLINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Create table entries
CREATE TABLE entries (
    entry_id UUID PRIMARY KEY,
    invoice_id UUID REFERENCES invoices(invoice_id),
    entry_type entry_type NOT NULL,
    description TEXT NOT NULL,
    value NUMERIC NOT NULL,
    due_date DATE NOT NULL,
    tag TEXT, 
    account_id TEXT,
    status entry_status NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE
);
