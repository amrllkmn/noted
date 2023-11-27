-- Add migration script here
CREATE TABLE IF NOT EXISTS notes (
  id UUID PRIMARY KEY,
  title varchar NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  UNIQUE (title)
);