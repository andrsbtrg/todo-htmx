-- Add migration script here

CREATE TABLE tickets(
	id SERIAL PRIMARY KEY,
	title TEXT NOT NULL,
	status TEXT,
	description TEXT,
	creator_id INTEGER
);
