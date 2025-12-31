-- This file is intentionally missing some tables for negative testing

BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS "database_release" (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  version TEXT NOT NULL UNIQUE,
  created_at INTEGER NOT NULL
) STRICT;
CREATE INDEX idx_database_release_created_at ON database_release(created_at);

COMMIT;
