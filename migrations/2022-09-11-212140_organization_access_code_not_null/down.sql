-- This file should undo anything in `up.sql`
ALTER TABLE organizations ALTER COLUMN access_code DROP NOT NULL;