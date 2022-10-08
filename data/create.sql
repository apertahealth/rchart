CREATE TABLE results (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    points INTEGER NOT NULL,
    tournament TEXT NOT NULL,
);

CREATE TABLE results (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    points INTEGER NOT NULL,
    tournament TEXT NOT NULL,
);
CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY,
    username TEXT NOT NULL,
    email INTEGER NOT NULL,
    phone TEXT NOT NULL,
);

CREATE TABLE [IF NOT EXISTS] users (
    id INTEGER NOT NULL PRIMARY KEY,
    username TEXT NOT NULL,
    password INTEGER NOT NULL,
    email TEXT NOT NULL UNIQUE,
    phone TEXT NOT NULL UNIQUE,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    role TEXT NOT NULL
) [WITHOUT ROWID];

insert into results (name, points, tournament)
VALUES
("ben", 200, "2020"),
("kris", 400, "2021");


select * from results;