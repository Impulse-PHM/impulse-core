BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS "user" (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  birth_year INTEGER NOT NULL,
  birth_month INTEGER NOT NULL,
  birth_day INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  is_primary INTEGER NOT NULL CHECK (is_primary IN (0, 1)),
  UNIQUE (first_name, last_name, birth_year, birth_month, birth_day)
) STRICT;
CREATE INDEX idx_user_created_at ON user(created_at);
-- Partial index to enforce, at most, one primary user.
CREATE UNIQUE INDEX idx_one_primary_user_at_most
ON user(is_primary)
WHERE is_primary = 1;

CREATE TABLE IF NOT EXISTS "bioactive_agent_type" (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE
) STRICT;

CREATE TABLE IF NOT EXISTS "bioactive_agent" (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  user_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  quantity REAL NOT NULL,
  quantity_unit_id INTEGER NOT NULL,
  frequency_unit_id INTEGER NOT NULL,
  agent_type_id INTEGER NOT NULL,
  is_prescription INTEGER NOT NULL CHECK (is_prescription IN (0, 1)),
  created_at INTEGER NOT NULL,
  is_deleted INTEGER NOT NULL CHECK (is_deleted IN (0, 1)),
  UNIQUE (user_id, name),
  FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (quantity_unit_id) REFERENCES unit(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (frequency_unit_id) REFERENCES unit(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (agent_type_id) REFERENCES bioactive_agent_type(id) ON DELETE CASCADE ON UPDATE CASCADE
) STRICT;
CREATE INDEX idx_bioactive_agent_user_id ON bioactive_agent(user_id);
CREATE INDEX idx_bioactive_agent_name ON bioactive_agent(name);
CREATE INDEX idx_bioactive_agent_quantity_unit_id ON bioactive_agent(quantity_unit_id);
CREATE INDEX idx_bioactive_agent_frequency_unit_id ON bioactive_agent(frequency_unit_id);
CREATE INDEX idx_bioactive_agent_agent_type_id ON bioactive_agent(agent_type_id);
CREATE INDEX idx_bioactive_agent_created_at ON bioactive_agent(created_at);
CREATE INDEX idx_bioactive_agent_is_prescription ON bioactive_agent(is_prescription);
CREATE INDEX idx_bioactive_agent_is_deleted ON bioactive_agent(is_deleted);

CREATE TABLE IF NOT EXISTS "bioactive_agent_optional_information" (
  agent_id INTEGER NOT NULL PRIMARY KEY,
  reason TEXT,
  notes TEXT,
  FOREIGN KEY (agent_id) REFERENCES bioactive_agent(id) ON DELETE CASCADE ON UPDATE CASCADE
) STRICT;

CREATE TABLE IF NOT EXISTS "bioactive_agent_log" (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  agent_id INTEGER NOT NULL,
  quantity REAL NOT NULL,
  created_at INTEGER NOT NULL,
  is_deleted INTEGER NOT NULL (is_deleted IN (0, 1)),
  UNIQUE (agent_id, created_at),
  FOREIGN KEY (agent_id) REFERENCES bioactive_agent(id) ON DELETE CASCADE ON UPDATE CASCADE
) STRICT;
CREATE INDEX idx_bioactive_agent_log_agent_id ON bioactive_agent_log(agent_id);
CREATE INDEX idx_bioactive_agent_log_created_at ON bioactive_agent_log(created_at);
CREATE INDEX idx_bioactive_agent_log_is_deleted ON bioactive_agent_log(is_deleted);

CREATE TABLE IF NOT EXISTS "bioactive_agent_log_optional_information" (
  log_id INTEGER NOT NULL PRIMARY KEY,
  notes TEXT,
  FOREIGN KEY (log_id) REFERENCES bioactive_agent_log(id) ON DELETE CASCADE ON UPDATE CASCADE
) STRICT;

CREATE TABLE IF NOT EXISTS "bioactive_agent_group" (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  is_deleted INTEGER NOT NULL (is_deleted IN (0, 1))
) STRICT;
CREATE INDEX idx_bioactive_agent_group_is_deleted ON bioactive_agent_group(is_deleted);

CREATE TABLE IF NOT EXISTS "bioactive_agent_group_member" (
  group_id INTEGER NOT NULL,
  agent_id INTEGER NOT NULL,
  PRIMARY KEY (group_id, agent_id),
  FOREIGN KEY (group_id) REFERENCES bioactive_agent_group(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (agent_id) REFERENCES bioactive_agent(id) ON DELETE CASCADE ON UPDATE CASCADE
) STRICT;
CREATE INDEX idx_bioactive_agent_group_member_group_id ON bioactive_agent_group_member(group_id);
CREATE INDEX idx_bioactive_agent_group_member_agent_id ON bioactive_agent_group_member(agent_id);

CREATE TABLE IF NOT EXISTS "unit" (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  singular_name TEXT NOT NULL UNIQUE,
  plural_name TEXT NOT NULL UNIQUE,
  abbreviation TEXT UNIQUE -- Not all units have an abbreviation
) STRICT;

CREATE TABLE IF NOT EXISTS "unit_category" (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE
) STRICT;

CREATE TABLE IF NOT EXISTS "categorized_unit" (
  unit_id INTEGER NOT NULL,
  category_id INTEGER NOT NULL,
  PRIMARY KEY (unit_id, category_id),
  FOREIGN KEY (unit_id) REFERENCES unit(id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (category_id) REFERENCES unit_category(id) ON DELETE CASCADE ON UPDATE CASCADE
) STRICT;
CREATE INDEX idx_categorized_unit_unit_id ON categorized_unit(unit_id);
CREATE INDEX idx_categorized_unit_category_id ON categorized_unit(category_id);

CREATE TABLE IF NOT EXISTS "database_release" (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  version TEXT NOT NULL UNIQUE,
  created_at INTEGER NOT NULL
) STRICT;
CREATE INDEX idx_database_release_created_at ON database_release(created_at);

COMMIT;
