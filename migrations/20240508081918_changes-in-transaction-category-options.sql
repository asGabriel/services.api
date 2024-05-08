ALTER TABLE
    transactions
ALTER COLUMN
    category TYPE TEXT;

DROP TYPE category;

CREATE TYPE category AS ENUM(
    'FOOD',
    'HOME',
    'EDUCATION',
    'ENTERTAINMENT',
    'TRANSPORT',
    'HEALTHY',
    'SALARY',
    'UTILITIES',
    'INSURANCE',
    'SAVINGS',
    'DEBT_PAYMENTS',
    'CHILD_CARE',
    'GIFTS',
    'SUBSCRIPTIONS',
    'TRAVEL',
    'CLOTHING',
    'MAINTENANCE'
);

ALTER TABLE
    transactions
ALTER COLUMN
    category TYPE category USING category :: category;