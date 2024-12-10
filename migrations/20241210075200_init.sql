CREATE TABLE IF NOT EXISTS Users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TYPE AddressType AS ENUM ('ethereum-like', 'solana');
CREATE TABLE IF NOT EXISTS OnChainAccounts (
        id SERIAL PRIMARY KEY,
    account_address VARCHAR(255) NOT NULL,
    address_type AddressType NOT NULL,
    user_id INT REFERENCES Users(id) ON DELETE CASCADE
);
