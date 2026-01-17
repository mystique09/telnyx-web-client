CREATE TABLE
    "users" (
        id UUID NOT NULL PRIMARY KEY,
        email TEXT NOT NULL UNIQUE,
        hash TEXT NOT NULL,
        salt TEXT NOT NULL,
        email_verified BOOLEAN NOT NULL,
        email_verified_at TIMESTAMPTZ NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT users_email_unique UNIQUE (email)
    );

CREATE INDEX users_email_idx ON users (email);

CREATE TABLE
    "phone_numbers" (
        id UUID NOT NULL PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        phone TEXT NOT NULL UNIQUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE INDEX phone_numbers_name_idx ON phone_numbers (name);

CREATE INDEX phone_numbers_phone_idx ON phone_numbers (phone);

CREATE TABLE
    "messages" (
        id UUID NOT NULL PRIMARY KEY,
        user_id UUID NOT NULL,
        content TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

ALTER TABLE "messages" ADD CONSTRAINT "fk-messages-user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE;