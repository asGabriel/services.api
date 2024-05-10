ALTER TABLE
    transactions
ALTER COLUMN
    month_reference TYPE TEXT;

DROP TYPE month_reference;

CREATE TYPE month_reference AS ENUM(
    'JANUARY',
    'FEBRUARY',
    'MARCH',
    'APRIL',
    'MAY',
    'JUNE',
    'JULY',
    'AUGUST',
    'SEPTEMBER',
    'OCTOBER',
    'NOVEMBER',
    'DECEMBER'
);

ALTER TABLE
    transactions
ALTER COLUMN
    month_reference TYPE month_reference USING month_reference :: month_reference;