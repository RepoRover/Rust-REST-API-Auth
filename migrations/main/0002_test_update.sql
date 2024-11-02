-- Since new features might be released and they require
-- database changes, we will just need to write SQL code to update
-- already existing schemas from previous persions
ALTER TABLE accounts
ADD COLUMN bio TEXT;