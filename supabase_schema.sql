-- Steam Account Tracker Database Schema
-- This schema supports tracking Steam accounts and detecting connections between them

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table - represents unique devices/machines
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_fingerprint TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    notes TEXT
);

-- Steam accounts table - all discovered Steam accounts
CREATE TABLE steam_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    steam_id TEXT NOT NULL,
    nickname TEXT,
    name_history TEXT[] DEFAULT '{}', -- Array of previous nicknames
    sources TEXT[] DEFAULT '{}', -- Sources like "registry", "libraryfolders.vdf", etc.
    most_recent BOOLEAN DEFAULT FALSE,
    time_stamp TIMESTAMP WITH TIME ZONE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Account connections table - links between potentially related accounts
CREATE TABLE account_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    primary_steam_id TEXT NOT NULL,
    linked_steam_id TEXT NOT NULL,
    confidence FLOAT NOT NULL CHECK (confidence >= 0 AND confidence <= 1),
    reason TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(primary_steam_id, linked_steam_id)
);

-- Indexes for performance
CREATE INDEX idx_steam_accounts_steam_id ON steam_accounts(steam_id);
CREATE INDEX idx_steam_accounts_user_id ON steam_accounts(user_id);
CREATE INDEX idx_steam_accounts_most_recent ON steam_accounts(most_recent) WHERE most_recent = true;
CREATE INDEX idx_account_connections_primary ON account_connections(primary_steam_id);
CREATE INDEX idx_account_connections_linked ON account_connections(linked_steam_id);
CREATE INDEX idx_account_connections_confidence ON account_connections(confidence DESC);

-- Function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger to automatically update updated_at
CREATE TRIGGER update_steam_accounts_updated_at 
    BEFORE UPDATE ON steam_accounts 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Row Level Security policies (optional but recommended)
-- Enable RLS
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE steam_accounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE account_connections ENABLE ROW LEVEL SECURITY;

-- Example policies (you'll need to adjust based on your auth setup)
-- These policies allow users to only see their own data
CREATE POLICY "Users can view their own data" ON users
    FOR ALL USING (auth.uid()::text = device_fingerprint OR auth.role() = 'service_role');

CREATE POLICY "Users can view their steam accounts" ON steam_accounts
    FOR ALL USING (
        user_id IN (
            SELECT id FROM users WHERE device_fingerprint = auth.uid()::text
        ) OR auth.role() = 'service_role'
    );

CREATE POLICY "Users can view account connections" ON account_connections
    FOR ALL USING (
        primary_steam_id IN (
            SELECT steam_id FROM steam_accounts WHERE user_id IN (
                SELECT id FROM users WHERE device_fingerprint = auth.uid()::text
            )
        ) OR 
        linked_steam_id IN (
            SELECT steam_id FROM steam_accounts WHERE user_id IN (
                SELECT id FROM users WHERE device_fingerprint = auth.uid()::text
            )
        ) OR 
        auth.role() = 'service_role'
    );

-- Useful views for analysis
CREATE VIEW account_connection_summary AS
SELECT 
    ac.primary_steam_id,
    ac.linked_steam_id,
    ac.confidence,
    ac.reason,
    sa1.nickname as primary_nickname,
    sa2.nickname as linked_nickname,
    sa1.user_id as primary_user_id,
    sa2.user_id as linked_user_id
FROM account_connections ac
JOIN steam_accounts sa1 ON ac.primary_steam_id = sa1.steam_id
JOIN steam_accounts sa2 ON ac.linked_steam_id = sa2.steam_id;

-- View for high-confidence connections
CREATE VIEW high_confidence_connections AS
SELECT * FROM account_connection_summary
WHERE confidence >= 0.7
ORDER BY confidence DESC;

-- Sample queries for analysis:

-- Find all accounts connected to a specific Steam ID
/*
SELECT DISTINCT
    CASE 
        WHEN primary_steam_id = '76561198000000000' THEN linked_steam_id
        ELSE primary_steam_id
    END as connected_steam_id,
    confidence,
    reason
FROM account_connections
WHERE primary_steam_id = '76561198000000000' OR linked_steam_id = '76561198000000000'
ORDER BY confidence DESC;
*/

-- Find potential ban evaders (accounts with high confidence connections)
/*
SELECT 
    primary_steam_id,
    COUNT(linked_steam_id) as connected_accounts,
    AVG(confidence) as avg_confidence,
    MAX(confidence) as max_confidence
FROM account_connections
WHERE confidence >= 0.5
GROUP BY primary_steam_id
HAVING COUNT(linked_steam_id) > 1
ORDER BY avg_confidence DESC, connected_accounts DESC;
*/

-- Find accounts with shared name history
/*
SELECT 
    sa1.steam_id as account1,
    sa2.steam_id as account2,
    sa1.nickname as nickname1,
    sa2.nickname as nickname2,
    sa1.name_history && sa2.name_history as has_shared_history
FROM steam_accounts sa1
CROSS JOIN steam_accounts sa2
WHERE sa1.steam_id < sa2.steam_id
AND sa1.name_history && sa2.name_history
AND array_length(sa1.name_history, 1) > 0
AND array_length(sa2.name_history, 1) > 0;
*/
