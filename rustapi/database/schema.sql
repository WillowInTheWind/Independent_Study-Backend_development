DROP TABLE IF EXISTS user;
DROP TABLE IF EXISTS MX;
DROP TABLE IF EXISTS user_sessions;
DROP TABLE IF EXISTS oauth2_state_storage;


CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_name TEXT UNIQUE NOT NULL,
    user_identifier INTEGER NOT NULL,
    user_email TEXT UNIQUE NOT NULL
);

INSERT into user values
(1,'chase', 128474, 'cwayland@fwparker.org'),
(2, 'willow', 1282334,'wayland.chase@gmail.com');

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
"id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
"user_id" integer NOT NULL,
"session_token_p1" text NOT NULL,
"session_token_p2" text NOT NULL,
"created_at" integer NOT NULL,
"expires_at" integer NOT NULL
);
CREATE TABLE "oauth2_state_storage" (
"id" integer NOT NULL PRIMARY KEY AUTOINCREMENT,
"csrf_state" text NOT NULL,
"pkce_code_verifier" text NOT NULL,
"return_url" text NOT NULL
);
