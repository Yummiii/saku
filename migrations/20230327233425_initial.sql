-- Add migration script here
CREATE TABLE "Context" (
	"id"	INTEGER NOT NULL,
	"role"	TEXT NOT NULL,
	"content"	TEXT NOT NULL,
	"channel" INTEGER NOT NULL,
	"name" TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "Channels" (
	"id"	INTEGER NOT NULL,
	"enabled" INTEGER NOT NULL,
	PRIMARY KEY("id")
);