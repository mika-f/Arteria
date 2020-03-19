-- This file should undo anything in `up.sql`
ALTER TABLE instances RENAME COLUMN executor_id TO version_id;
ALTER TABLE executors RENAME TO versions;
