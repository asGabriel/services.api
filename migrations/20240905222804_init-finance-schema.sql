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
    is_main BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX unique_parent_per_month_year
ON invoices (month, year)
WHERE is_main = true;

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
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    status entry_status NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- TAGS TABLE
CREATE TABLE tags(
    tag_id INT PRIMARY KEY,
    value TEXT NOT NULL UNIQUE
);

-- ENTRIES TAGS TABLE
CREATE TABLE entries_tags (
    entry_id UUID NOT NULL REFERENCES entries(entry_id),
    tag_id INT NOT NULL REFERENCES tags(tag_id),
    PRIMARY KEY (entry_id, tag_id)
);

-- SUB INVOICES TABLE
CREATE TABLE invoice_relations (
    parent_invoice_id UUID NOT NULL,
    child_invoice_id UUID NOT NULL,
    PRIMARY KEY (parent_invoice_id, child_invoice_id),
    FOREIGN KEY (parent_invoice_id) REFERENCES invoices(invoice_id),
    FOREIGN KEY (child_invoice_id) REFERENCES invoices(invoice_id)
);

-- INDEXES
-- INVOICES TABLE
CREATE INDEX idx_invoices_month_year ON invoices (month, year);
CREATE INDEX idx_invoices_is_main ON invoices (is_main);

-- ENTRIES TABLE
CREATE INDEX idx_entries_invoice_id ON entries (invoice_id);
CREATE INDEX idx_entries_entry_type ON entries (entry_type);
CREATE INDEX idx_entries_due_date ON entries (due_date);
CREATE INDEX idx_entries_account_id ON entries (account_id);
CREATE INDEX idx_entries_status ON entries (status);

-- INVOICE RELATIONS TABLES
CREATE INDEX idx_invoice_relations_parent ON invoice_relations (parent_invoice_id);
CREATE INDEX idx_invoice_relations_child ON invoice_relations (child_invoice_id);

-- TAGS
CREATE INDEX idx_tags_value ON tags (value);

-- ENTRIES TAGS
CREATE INDEX idx_entries_tags_entry_id ON entries_tags (entry_id);
CREATE INDEX idx_entries_tags_tag_id ON entries_tags (tag_id);
