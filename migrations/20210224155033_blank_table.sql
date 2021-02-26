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
   password TEXT NOT NULL,
   subscribed_at timestamptz NOT NULL
);