-- -*- mode: sql -*-

CREATE TABLE IF NOT EXISTS items (
  id   TEXT PRIMARY KEY,
  name TEXT,
  type TEXT -- needs to become FK to type table
);
