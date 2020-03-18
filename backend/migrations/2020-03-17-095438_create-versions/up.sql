-- Your SQL goes here
CREATE TABLE versions (
  id INTEGER NOT NULL AUTO_INCREMENT,
  name VARCHAR(32) NOT NULL,
  tag VARCHAR(32) NOT NULL,
  PRIMARY KEY(id),
  INDEX versions_tag_idx(tag)
);