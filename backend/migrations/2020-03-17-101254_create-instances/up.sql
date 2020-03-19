-- Your SQL goes here
CREATE TABLE instances (
  id BIGINT AUTO_INCREMENT,
  title VARCHAR(128) NOT NULL DEFAULT 'Untitled',
  version_id INT NOT NULL,
  status ENUM('running', 'success', 'failure', 'terminate') NOT NULL,
  result TEXT,
  CONSTRAINT fk_instances_version_id
    FOREIGN KEY (version_id)
    REFERENCES versions (id)
    ON DELETE RESTRICT
    ON UPDATE RESTRICT,
  PRIMARY KEY(id)
);