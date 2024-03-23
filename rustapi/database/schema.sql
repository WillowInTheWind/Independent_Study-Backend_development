DROP TABLE IF EXISTS user;

CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_name TEXT UNIQUE NOT NULL,
    user_identifier INTEGER NOT NULL
);

INSERT into user values
(1,'chase', 128474),
(2, 'willow', 1282334)
