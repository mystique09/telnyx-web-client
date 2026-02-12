DO $$
BEGIN
    IF NOT EXISTS (
        SELECT
            1
        FROM
            pg_type
        WHERE
            typname = 'message_type'
    ) THEN
        CREATE TYPE message_type AS ENUM ('INBOUND', 'OUTBOUND');
    END IF;
END
$$;

ALTER TABLE "phone_numbers" ADD COLUMN user_id UUID;

DO $$
BEGIN
    IF EXISTS (
        SELECT
            1
        FROM
            phone_numbers
        WHERE
            user_id IS NULL
    ) THEN
        RAISE EXCEPTION 'Cannot enforce NOT NULL on phone_numbers.user_id without backfilling existing rows.';
    END IF;
END
$$;

ALTER TABLE "phone_numbers" ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE "phone_numbers" ADD CONSTRAINT "fk-phone_numbers-user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE;
ALTER TABLE "phone_numbers" DROP CONSTRAINT IF EXISTS phone_numbers_name_key;
ALTER TABLE "phone_numbers" ADD CONSTRAINT phone_numbers_user_id_name_unique UNIQUE (user_id, name);
CREATE INDEX phone_numbers_user_id_idx ON phone_numbers (user_id);

CREATE TABLE
    "conversations" (
        id UUID NOT NULL PRIMARY KEY,
        phone_number_id UUID NOT NULL,
        user_id UUID NOT NULL,
        last_message_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

ALTER TABLE "conversations" ADD CONSTRAINT "fk-conversations-phone_number_id" FOREIGN KEY ("phone_number_id") REFERENCES "phone_numbers" ("id") ON DELETE CASCADE;
ALTER TABLE "conversations" ADD CONSTRAINT "fk-conversations-user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE;
ALTER TABLE "conversations" ADD CONSTRAINT conversations_id_user_id_unique UNIQUE (id, user_id);

CREATE INDEX conversations_user_last_message_idx ON conversations (user_id, last_message_at DESC, id DESC);
CREATE INDEX conversations_phone_number_idx ON conversations (phone_number_id);

ALTER TABLE "messages" ADD COLUMN conversation_id UUID;
ALTER TABLE "messages" ADD COLUMN from_number TEXT;
ALTER TABLE "messages" ADD COLUMN message_type message_type NOT NULL DEFAULT 'OUTBOUND';

DO $$
BEGIN
    IF EXISTS (
        SELECT
            1
        FROM
            messages
        WHERE
            conversation_id IS NULL
    ) THEN
        RAISE EXCEPTION 'Cannot enforce NOT NULL on messages.conversation_id without backfilling existing rows.';
    END IF;
END
$$;

UPDATE "messages"
SET
    from_number = ''
WHERE
    from_number IS NULL;

ALTER TABLE "messages" ALTER COLUMN conversation_id SET NOT NULL;
ALTER TABLE "messages" ALTER COLUMN from_number SET NOT NULL;
ALTER TABLE "messages" ADD CONSTRAINT "fk-messages-conversation_id" FOREIGN KEY ("conversation_id") REFERENCES "conversations" ("id") ON DELETE CASCADE;
ALTER TABLE "messages" ADD CONSTRAINT "fk-messages-conversation_id-user_id" FOREIGN KEY ("conversation_id", "user_id") REFERENCES "conversations" ("id", "user_id") ON DELETE CASCADE;

-- Keyset pagination for message history:
-- SELECT ... FROM messages
-- WHERE conversation_id = $1 AND (created_at, id) < ($2, $3)
-- ORDER BY created_at DESC, id DESC
-- LIMIT $4;
CREATE INDEX messages_conversation_created_at_id_desc_idx ON messages (conversation_id, created_at DESC, id DESC);
