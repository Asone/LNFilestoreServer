-- Your SQL goes here

CREATE TABLE IF NOT EXISTS "media_payment" (
    "uuid" uuid UNIQUE NOT NULL,
    "request" text UNIQUE NOT NULL,
    "state" text,
    "hash" TEXT UNIQUE NOT NULL,
    "media_uuid" uuid references media(uuid) NOT NULL,
    "expires_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    "valid_until" TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    PRIMARY KEY( uuid )
);