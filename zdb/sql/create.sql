PRAGMA foreign_keys = ON;

DROP TABLE IF EXISTS servers;
DROP TABLE IF EXISTS streams;
DROP TABLE IF EXISTS topics;


CREATE TABLE servers (
	server_id INTEGER PRIMARY KEY NOT NULL,
	server_name TEXT NOT NULL,
	server_url TEXT NOT NULL,
	server_logo BLOB NOT NULL
);

CREATE TABLE streams (
	stream_id INTEGER PRIMARY KEY NOT NULL,
	stream_server INTEGER,
	name TEXT,
	description TEXT,
	FOREIGN KEY(stream_server) REFERENCES servers(server_id)
);

CREATE TABLE topics (
	topic_stream INTEGER,
	topic_id INTEGER PRIMARY KEY NOT NULL,
	name TEXT,
	FOREIGN KEY(topic_stream) REFERENCES streams(stream_id)
);

