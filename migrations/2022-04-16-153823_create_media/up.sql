-- Your SQL goes here-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "media" (
    "uuid" uuid UNIQUE PRIMARY KEY NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT,
    "absolute_path" TEXT NOT NULL,
    "price" INT NOT NULL DEFAULT 0,
    "payment_duration" INT DEFAULT NULL,
    "published" boolean NOT NULL DEFAULT false,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
