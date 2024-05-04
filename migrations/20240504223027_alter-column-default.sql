ALTER TABLE transactions
ALTER COLUMN description SET NOT NULL,
ALTER COLUMN description SET DEFAULT 'N/A';
