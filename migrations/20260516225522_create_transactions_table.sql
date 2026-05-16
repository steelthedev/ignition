-- Add migration script here


CREATE TABLE transactions(
  id UUID PRIMARY KEY,
  payload TEXT NOT NULL,
  status TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
