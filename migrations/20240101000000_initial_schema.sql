DO $$ BEGIN
    CREATE TYPE userstatus AS ENUM ('Online', 'Away', 'Offline');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_online TIMESTAMPTZ NOT NULL DEFAULT now(),
    status userstatus NOT NULL DEFAULT 'Offline',
    bio TEXT
);
