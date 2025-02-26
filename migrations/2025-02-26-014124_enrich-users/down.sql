-- This file should undo anything in `up.sql`
ALTER TABLE users
    DROP COLUMN fingerprint,
    DROP COLUMN timezone_offset,
    DROP COLUMN favorite_team,
    DROP COLUMN dark_mode;
