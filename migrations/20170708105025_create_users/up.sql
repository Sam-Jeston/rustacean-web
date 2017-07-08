CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(256) NOT NULL,
  password VARCHAR(256) NOT NULL,
  -- this is sad, but diesel does not support timestamps for mysql at this point in time
  created_at VARCHAR(256) NOT NULL,
  updated_at VARCHAR(256) NOT NULL
)
