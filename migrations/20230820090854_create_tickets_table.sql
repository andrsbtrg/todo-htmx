-- Add migration script here
CREATE TABLE tickets(
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	title TEXT NOT NULL,
	status TEXT
);
