ALTER TABLE
    reccurence_transactions
ADD
    COLUMN category category NOT NULL;

ALTER TABLE
    reccurence_transactions
ADD
    COLUMN due_date DATE NOT NULL;