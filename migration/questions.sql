CREATE TABLE IF NOT EXISTS questions (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    account_id serial,
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
); 