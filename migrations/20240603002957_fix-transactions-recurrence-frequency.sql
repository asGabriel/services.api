ALTER TABLE transactions ALTER COLUMN recurrence_frequency TYPE TEXT;

DROP TYPE transaction_recurrency;

CREATE TYPE recurrence_frequency AS ENUM(
    'SINGLE_OCCURRENCE',
    'WEEKLY',
    'MONTHLY',
    'QUARTERLY',
    'SEMI_ANUALLY',
    'ANUALLY'
);

ALTER TABLE transactions ALTER COLUMN recurrence_frequency TYPE recurrence_frequency USING recurrence_frequency :: recurrence_frequency;

ALTER TABLE transactions ALTER COLUMN recurrence_frequency SET NOT NULL;
