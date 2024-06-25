CREATE TABLE IF NOT EXISTS answers (
    id serial PRIMARY KEY,
    content TEXT NOT NULL,
    account_id serial,
    created_on TIMESTAMP NOT NULL DEFAULT NOW(),
    corresponding_question integer REFERENCES questions
);