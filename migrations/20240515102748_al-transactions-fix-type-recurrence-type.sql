ALTER TABLE transactions ALTER COLUMN recurrence_frequency TYPE TEXT;

DROP TYPE transaction_recurrency;

CREATE TYPE transaction_recurrency AS ENUM(
    'SINGLE_OCCURRENCE',
    'WEEKLY',
    'MONTHLY',
    'QUARTERLY',
    'SEMI_ANUALLY',
    'ANUALLY'
);

ALTER TABLE transactions ALTER COLUMN recurrence_frequency TYPE transaction_recurrency USING recurrence_frequency :: transaction_recurrency;

ALTER TABLE transactions ALTER COLUMN recurrence_frequency SET NOT NULL;
