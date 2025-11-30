-- Initial database setup for Imphnen Backend
-- This file is executed when PostgreSQL container starts for the first time

-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_stat_statements";

-- Create schemas for better organization
CREATE SCHEMA IF NOT EXISTS auth;
CREATE SCHEMA IF NOT EXISTS content;
CREATE SCHEMA IF NOT EXISTS gacha;
CREATE SCHEMA IF NOT EXISTS mentoring;

-- Set default privileges for future tables
ALTER DEFAULT PRIVILEGES IN SCHEMA auth GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO PUBLIC;
ALTER DEFAULT PRIVILEGES IN SCHEMA content GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO PUBLIC;
ALTER DEFAULT PRIVILEGES IN SCHEMA gacha GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO PUBLIC;
ALTER DEFAULT PRIVILEGES IN SCHEMA mentoring GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO PUBLIC;

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_created_at ON users(created_at) WHERE created_at IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_updated_at ON users(updated_at) WHERE updated_at IS NOT NULL;

-- Grant necessary permissions
GRANT USAGE ON SCHEMA auth TO PUBLIC;
GRANT USAGE ON SCHEMA content TO PUBLIC;
GRANT USAGE ON SCHEMA gacha TO PUBLIC;
GRANT USAGE ON SCHEMA mentoring TO PUBLIC;

-- Set timezone
SET timezone = 'UTC';