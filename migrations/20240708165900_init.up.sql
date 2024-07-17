-- Add migration script here


CREATE TABLE node.transaction(
    addr varchar(128) NOT NULL,
    epv_today numeric(10,2) NOT NULL DEFAULT 0.00,
    e_chg_today numeric(10,2) NOT NULL DEFAULT 0.00,
    e_dischg_today numeric(10,2) NOT NULL DEFAULT 0.00,
    signature varchar(255) NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    id  SERIAL NOT NULL
);