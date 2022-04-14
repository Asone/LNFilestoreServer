-- Your SQL goes here-- Your SQL goes here

CREATE TABLE IF NOT EXISTS "session" (
    "uuid" uuid UNIQUE PRIMARY KEY NOT NULL,
    "token" TEXT UNIQUE NOT NULL,
    "user_uuid" uuid NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expires_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT session FOREIGN KEY("user_uuid") REFERENCES "user"(uuid)
);
