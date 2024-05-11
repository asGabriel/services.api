ALTER TABLE
    installments
ADD
    COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE
    installments
ADD
    COLUMN updated_at TIMESTAMPTZ NOT NULL;

ALTER TABLE
    installments
ADD
    COLUMN deleted_at TIMESTAMPTZ NOT NULL;

