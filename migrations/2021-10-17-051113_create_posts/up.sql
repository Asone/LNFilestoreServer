-- Your SQL goes here

CREATE TABLE IF NOT EXISTS "post" (
        "uuid" uuid UNIQUE NOT NULL,
        "title" text NOT NULL,
        "excerpt" text NOT NULL,
        "content" text NOT NULL,
        "published" boolean NOT NULL DEFAULT false,
        "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
        "price" INT NOT NULL DEFAULT 0,
        PRIMARY KEY( uuid )
);

CREATE TABLE IF NOT EXISTS "payment" (
    "uuid" uuid UNIQUE NOT NULL,
    "request" text UNIQUE NOT NULL,
    "state" text,
    "hash" TEXT UNIQUE NOT NULL,
    "post_uuid" uuid references post(uuid) NOT NULL,
    PRIMARY KEY( uuid )
);

