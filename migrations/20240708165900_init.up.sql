-- Add migration script here


CREATE TABLE
    IF NOT EXISTS node.transaction (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        addr VARCHAR(128) NOT NULL UNIQUE,
        epv_today DECIMAL(10,2) DEFAULT 0.00 NOT NULL  ,
        e_chg_today DECIMAL(10,2) DEFAULT 0.00 NOT NULL ,
        e_dischg_today DECIMAL(10,2) DEFAULT 0.00 NOT NULL ,
        signature VARCHAR(255) NOT NULL UNIQUE,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );