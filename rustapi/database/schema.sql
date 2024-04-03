DROP TABLE IF EXISTS user;
DROP TABLE IF EXISTS GoogleUsers;
DROP TABLE IF EXISTS MX;
DROP TABLE IF EXISTS user_sessions;
DROP TABLE IF EXISTS oauth2_state_storage;


CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    user_identifier INTEGER NOT NULL,
    user_email TEXT UNIQUE NOT NULL
);

CREATE TABLE GoogleUsers (
    id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    sub TEXT UNIQUE NOT NULL,
    picture TEXT,
    email TEXT UNIQUE NOT NULL,
    name TEXT UNIQUE NOT NULL
);


CREATE TABLE MX (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mx_index INTEGER NOT NULL,
    date DATE UNIQUE NOT NULL,
    owner INTEGER,
    title TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
    FOREIGN KEY(owner) REFERENCES user(id)
);

CREATE TABLE "user_sessions" (
"id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
"user_id" INTEGER NOT NULL,
"session_token_p1" TEXT NOT NULL,
"session_token_p2" TEXT NOT NULL,
"created_at" INTEGER NOT NULL,
"expires_at" INTEGER NOT NULL
);
CREATE TABLE "oauth2_state_storage" (
    "id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
    "csrf_state" text NOT NULL,
    "pkce_code_verifier" text NOT NULL,
    "return_url" text NOT NULL
);
