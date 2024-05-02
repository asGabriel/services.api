CREATE TYPE movement_type AS ENUM('INCOME', 'EXPENSE');

CREATE TYPE transaction_category AS ENUM(
    'FOOD',
    'HOME',
    'EDUCATION',
    'PERSONAL_EXPENSE',
    'SPORADIC',
    'ENTERTAINMENT',
    'TRANSPORT',
    'HEALTHY',
    'SALARY',
    'OTHER'
);

CREATE TYPE transaction_status AS ENUM ('PENDING', 'CANCELED', 'COMPLETED');

CREATE TYPE transaction_recurrency AS ENUM(
    'WEEKLY',
    'MONTHLY',
    'QUARTERLY',
    'SEMI_ANUALLY',
    'ANUALLY'
);

-- movement type
ALTER TABLE
    transactions
ADD
    COLUMN movement_type movement_type;

-- category
ALTER TABLE
    transactions
ALTER COLUMN
    category TYPE transaction_category USING category :: transaction_category;

-- status
ALTER TABLE
    transactions
ALTER COLUMN
    status DROP DEFAULT;

ALTER TABLE
    transactions
ALTER COLUMN
    status TYPE transaction_status USING status :: transaction_status;

-- recurrence frequency
ALTER TABLE
    transactions
ALTER COLUMN
    recurrence_frequency TYPE transaction_recurrency USING recurrence_frequency :: transaction_recurrency;