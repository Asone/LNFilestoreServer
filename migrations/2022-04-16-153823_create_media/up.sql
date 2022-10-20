-- Your SQL goes here-- Your SQL goes here
CREATE TYPE media_type AS ENUM ('Default', 'Audio', 'Video','Pdf','Epub','Image');

CREATE TABLE IF NOT EXISTS "file" (
    "uuid" uuid UNIQUE PRIMARY KEY NOT NULL,
    "absolute_path" TEXT NOT NULL,
    "uploaded_by" uuid NOT NULL,
    "checksum" TEXT NOT NULL,
    "size" INT NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "media" (
    "uuid" uuid UNIQUE PRIMARY KEY NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT,
    "price" INT NOT NULL DEFAULT 0,
    "published" boolean NOT NULL DEFAULT false,
    "file_uuid" uuid UNIQUE NOT NULL,
    "type" media_type NOT NULL DEFAULT 'Default',
    "metadata" uuid UNIQUE NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT media FOREIGN KEY("file_uuid") REFERENCES "file"(uuid)
);

CREATE TABLE IF NOT EXISTS "audio_metadata" (
    "uuid" uuid UNIQUE PRIMARY KEY NOT NULL,
    "codec" TEXT DEFAULT NULL,
    "length" TEXT DEFAULT NULL,
    "artist" TEXT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS "video_metadata" (
    "uuid" uuid UNIQUE PRIMARY KEY NOT NULL,
    "codec" TEXT DEFAULT NULL,
    "length" TEXT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS "image_metadata" (
    "uuid" uuid UNIQUE PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS "epub_metadata" (
    "uuid" uuid UNIQUE PRIMARY KEY NOT NULL
);



