ALTER TABLE "conversations" ADD COLUMN recipient_phone_number TEXT;

ALTER TABLE "messages" ADD COLUMN provider_message_id TEXT;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT
            1
        FROM
            pg_enum
        WHERE
            enumlabel = 'queued'
            AND enumtypid = 'message_status'::regtype
    ) THEN
        ALTER TYPE message_status ADD VALUE 'queued';
    END IF;
END
$$;
