-- Your SQL goes here
CREATE TABLE files (
  id BIGINT AUTO_INCREMENT,
  instance_id BIGINT NOT NULL,
  title VARCHAR(128) NOT NULL DEFAULT 'main.pl',
  content TEXT NOT NULL,
  CONSTRAINT fk_files_instance_id 
    FOREIGN KEY (instance_id)
    REFERENCES instances (id)
    ON DELETE RESTRICT
    ON UPDATE RESTRICT,
  PRIMARY KEY(id)
);