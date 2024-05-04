ALTER TABLE transactions ALTER COLUMN due_date SET NOT NULL;

ALTER TABLE transactions ALTER COLUMN category SET NOT NULL;

ALTER TABLE transactions ALTER COLUMN recurring SET NOT NULL;

ALTER TABLE transactions ALTER COLUMN recurrence_frequency SET NOT NULL;

ALTER TABLE transactions ALTER COLUMN recurrence_duration_months SET NOT NULL;

ALTER TABLE transactions ALTER COLUMN recurrence_duration_months SET NOT NULL;
