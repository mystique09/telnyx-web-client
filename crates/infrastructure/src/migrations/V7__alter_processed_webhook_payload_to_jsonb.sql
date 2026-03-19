ALTER TABLE "processed_webhook_events"
    ALTER COLUMN payload_json TYPE JSONB
    USING payload_json::jsonb;
