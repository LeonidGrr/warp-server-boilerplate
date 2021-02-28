-- Add migration script here
CREATE TABLE blank(
   id uuid NOT NULL,
   PRIMARY KEY (id)
);

CREATE TABLE users(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   email TEXT NOT NULL UNIQUE,
   name TEXT NOT NULL,
   hash TEXT NOT NULL,
   created_at timestamptz NOT NULL
);