-- Add migration script here
CREATE TABLE tickets(
	id INTEGER NOT NULL,
	title TEXT NOT NULL,
	created_at DATETIME NOT NULL,
	status TEXT
);
