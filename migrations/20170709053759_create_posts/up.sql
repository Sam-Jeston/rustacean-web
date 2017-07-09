CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR(256) NOT NULL,
  caption VARCHAR(256) NOT NULL,
  body TEXT NOT NULL,
  -- this is sad, but diesel does not support timestamps for mysql at this point in time
  created_at VARCHAR(256) NOT NULL,
  updated_at VARCHAR(256) NOT NULL
)
