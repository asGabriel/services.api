CREATE TYPE month_reference AS ENUM(
    '01',
    '02',
    '03',
    '04',
    '05',
    '06',
    '07',
    '08',
    '09',
    '10',
    '11',
    '12'
);

ALTER TABLE
    transactions
ADD
    COLUMN month_reference month_reference;