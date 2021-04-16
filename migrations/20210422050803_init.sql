-- Add migration script here
CREATE TABLE emails (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"to" TEXT NOT NULL,
	"from" TEXT NOT NULL,
	"text" TEXT NOT NULL,
	html TEXT NOT NULL,
	sender_ip TEXT NOT NULL,
	webhook_request_ip TEXT NOT NULL,
	subject TEXT NOT NULL,
	dkim TEXT NOT NULL,
	created_at REAL NOT NULL
);

CREATE INDEX emails_to_IDX ON emails ("to");
CREATE INDEX emails_created_at_IDX ON emails (created_at);
CREATE INDEX emails_to_created_at_IDX ON emails ("to",created_at);
