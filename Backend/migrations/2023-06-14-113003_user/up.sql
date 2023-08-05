-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  user_name VARCHAR NOT NULL,
  user_email VARCHAR NOT NULL,
  user_password VARCHAR NOT NULL
)