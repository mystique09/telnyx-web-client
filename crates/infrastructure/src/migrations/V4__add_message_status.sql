DO $$
BEGIN
    IF NOT EXISTS (
        SELECT
            1
        FROM
            pg_type
        WHERE
            typname = 'message_status'
    ) THEN
        CREATE TYPE message_status AS ENUM ('pending', 'delivered', 'failed', 'sent');
    END IF;
END
$$;

ALTER TABLE "messages" ADD COLUMN status message_status NOT NULL DEFAULT 'pending';

CREATE INDEX messages_conversation_status_idx ON messages (conversation_id, status);
