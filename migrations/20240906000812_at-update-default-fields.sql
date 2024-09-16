ALTER TABLE invoices
    ALTER COLUMN created_at SET DEFAULT now(),
    ALTER COLUMN updated_at DROP DEFAULT,
    ALTER COLUMN deleted_at DROP DEFAULT;

ALTER TABLE entries
    ALTER COLUMN created_at SET DEFAULT now(),
    ALTER COLUMN updated_at DROP DEFAULT,
    ALTER COLUMN deleted_at DROP DEFAULT;
