-- Add migration script here
CREATE TABLE contents (
    content_id SERIAL PRIMARY KEY,
    content_title varchar(160) NOT NULL,
    content_link varchar(160) NULL,
    content_short int NOT NULL DEFAULT 0,
    content_number int NOT NULL,
    content_sub int NULL,
    content_parrent int NULL,
    content_create_at Timestamp
);