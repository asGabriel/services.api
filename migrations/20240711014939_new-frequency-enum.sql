ALTER TABLE
    recurrences DROP COLUMN frequency;

CREATE TYPE frequency AS ENUM('WEEKLY', 'MONTHLY', 'ANUALLY');

DROP TYPE recurrence_frequency;

ALTER TABLE
    recurrences
ADD
    COLUMN frequency frequency NOT NULL;