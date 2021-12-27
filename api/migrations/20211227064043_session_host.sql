-- Add migration script here
ALTER TABLE sessions 
    ADD COLUMN started boolean NOT NULL DEFAULT false;

ALTER TABLE players
    ADD COLUMN host boolean NOT NULL DEFAULT false;