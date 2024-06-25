ALTER TABLE recurrence_transactions ADD COLUMN reference INTEGER NOT NULL;

ALTER TABLE recurrence_transactions DROP COLUMN due_date;
