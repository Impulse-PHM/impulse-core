BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS "app_log" (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  level TEXT NOT NULL,
  message TEXT NOT NULL,
  created_at INTEGER NOT NULL
) STRICT;
CREATE INDEX idx_app_log_created_at ON app_log(created_at);

CREATE TABLE IF NOT EXISTS "database_release" (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  version TEXT NOT NULL UNIQUE,
  created_at INTEGER NOT NULL
) STRICT;
CREATE INDEX idx_database_release_created_at ON database_release(created_at);

COMMIT;
