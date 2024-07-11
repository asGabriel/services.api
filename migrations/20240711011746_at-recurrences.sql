ALTER TABLE
    recurrence_transactions RENAME TO recurrences;

ALTER TABLE
    recurrences DROP COLUMN amount;

ALTER TABLE
    recurrences
ADD
    COLUMN value NUMERIC NOT NULL;

ALTER TABLE
    recurrences RENAME COLUMN description TO title;

ALTER TABLE
    recurrences
ALTER COLUMN
    start_date DROP DEFAULT;

ALTER TABLE
    recurrences DROP COLUMN reference;