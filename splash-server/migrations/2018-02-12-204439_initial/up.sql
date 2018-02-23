CREATE TABLE users (
  "id" SERIAL PRIMARY KEY,
  "name" VARCHAR NOT NULL,
  "pw_hash" VARCHAR NOT NULL,
  "salt" VARCHAR NOT NULL
);

CREATE TABLE sessions (
  "id" SERIAL PRIMARY KEY,
  "token" VARCHAR NOT NULL,
  "user" SERIAL NOT NULL,
  FOREIGN KEY ("user") REFERENCES users("id")
);

CREATE TABLE space_events (
  "id" SERIAL PRIMARY KEY,
  "open" BOOLEAN NOT NULL,
  "created_at" TIMESTAMP DEFAULT current_timestamp
);
