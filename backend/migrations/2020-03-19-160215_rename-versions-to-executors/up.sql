-- Your SQL goes here
ALTER TABLE versions RENAME TO executors;
ALTER TABLE instances RENAME COLUMN version_id TO executor_id;