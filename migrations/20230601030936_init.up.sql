-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS pets (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        name VARCHAR(255) NOT NULL UNIQUE,
        category VARCHAR(100),
        breed BOOLEAN DEFAULT FALSE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
