BEGIN TRANSACTION;

INSERT INTO database_release VALUES (1, '0.1.0', unixepoch('now'));

COMMIT;
