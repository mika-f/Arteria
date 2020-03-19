-- Your SQL goes here
CREATE TABLE dependencies (
  id BIGINT AUTO_INCREMENT,
  instance_id BIGINT NOT NULL,
  name_with_version VARCHAR(256) NOT NULL,
  CONSTRAINT fk_dependencies_instance_id
    FOREIGN KEY (instance_id)
    REFERENCES instances (id)
    ON DELETE RESTRICT
    ON UPDATE RESTRICT,
  PRIMARY KEY(id)
);