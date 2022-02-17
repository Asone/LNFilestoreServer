-- Your SQL goes here

CREATE TABLE IF NOT EXISTS "api_payment" (
    "uuid" uuid UNIQUE NOT NULL,
    "request" text UNIQUE NOT NULL,
    "state" text,
    "hash" TEXT UNIQUE NOT NULL,
    "expires_at" TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY( uuid )
);