-- Add migration script here
ALTER TABLE players ADD COLUMN finished boolean NOT NULL DEFAULT false; 