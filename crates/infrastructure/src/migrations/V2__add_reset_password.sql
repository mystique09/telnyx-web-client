CREATE TABLE
    "reset_passwords" (
        id UUID NOT NULL PRIMARY KEY,
        user_id UUID NOT NULL,
        token TEXT NOT NULL,
        consumed BOOLEAN NOT NULL DEFAULT FALSE,
        consumed_at TIMESTAMPTZ,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT reset_passwords_token_unique UNIQUE (token)
    );

ALTER TABLE "reset_passwords" ADD CONSTRAINT "fk-reset_passwords-user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE;