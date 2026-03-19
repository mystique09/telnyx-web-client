ALTER TABLE "messages" ADD COLUMN provider_status TEXT;
ALTER TABLE "messages" ADD COLUMN provider_status_updated_at TIMESTAMPTZ;
ALTER TABLE "messages" ADD COLUMN provider_error_code TEXT;
ALTER TABLE "messages" ADD COLUMN provider_error_detail TEXT;

CREATE UNIQUE INDEX messages_provider_message_id_unique_idx
    ON messages (provider_message_id)
    WHERE provider_message_id IS NOT NULL;

CREATE INDEX conversations_user_phone_recipient_idx
    ON conversations (user_id, phone_number_id, recipient_phone_number);

CREATE TABLE
    "processed_webhook_events" (
        event_id TEXT NOT NULL PRIMARY KEY,
        event_type TEXT NOT NULL,
        provider_message_id TEXT,
        occurred_at TIMESTAMPTZ NOT NULL,
        payload_json TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE INDEX processed_webhook_events_provider_message_id_idx
    ON processed_webhook_events (provider_message_id);
