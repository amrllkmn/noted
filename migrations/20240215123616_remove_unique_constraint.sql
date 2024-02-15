-- Add migration script here
ALTER TABLE notes
DROP CONSTRAINT IF EXISTS notes_title_key;