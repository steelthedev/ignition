-- Add migration script here
--

ALTER TABLE transactions
ADD COLUMN retry_count INTEGER NOT NULL DEFAULT 0;


ALTER TABLE transactions
ADD COLUMN last_error TEXT;
