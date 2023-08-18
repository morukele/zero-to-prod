-- Add migration script here
-- The whole migration is wrapped in a transaction to make sure it succeds or fails atomically
BEGIN;
    -- Backfill `status` for historial entries
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    -- Make `status` mandatory
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;