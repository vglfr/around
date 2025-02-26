ALTER TABLE users
    ADD COLUMN fingerprint varchar NOT NULL,
    ADD COLUMN timezone_offset int,
    ADD COLUMN favorite_team varchar,
    ADD COLUMN dark_mode boolean;
