-- This file uses an intentionally invalid schema version for negative testing

BEGIN TRANSACTION;

-- This project expects 0.1.0 and higher as recommended in the semantic 
-- versioning rules.
INSERT INTO database_release VALUES (1, '0.0.0', unixepoch('now'));

COMMIT;
