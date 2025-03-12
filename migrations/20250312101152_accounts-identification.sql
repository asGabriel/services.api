ALTER TABLE accounts ADD COLUMN identification TEXT;

UPDATE accounts SET identification = 'Pessoa física' WHERE account_id = '156abfe9-2b55-44b0-abd3-460a61ed1342';

ALTER TABLE accounts ALTER COLUMN identification SET NOT NULL;
