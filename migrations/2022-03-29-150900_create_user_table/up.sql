-- Your SQL goes here
CREATE TYPE user_role AS ENUM ('Admin','Moderator','Publisher');

CREATE TABLE IF NOT EXISTS "user" (
    "uuid" uuid UNIQUE NOT NULL,
    "login" TEXT UNIQUE NOT NULL,
    "email" TEXT UNIQUE NOT NULL,
    "password" TEXT NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "role" user_role NOT NULL DEFAULT 'Publisher',
    PRIMARY KEY( uuid )
);
