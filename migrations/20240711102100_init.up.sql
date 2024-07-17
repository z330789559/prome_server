-- Add migration script here


alter TABLE transaction add COLUMN  start_ts bigint NOT NULL DEFAULT 0;
alter TABLE transaction add COLUMN  end_ts bigint NOT NULL DEFAULT 0;
alter TABLE transaction add COLUMN  egridin_today numeric(10,2) NOT NULL DEFAULT 0.00;
alter TABLE transaction add COLUMN  egridout_today numeric(10,2) NOT NULL DEFAULT 0.00;
alter TABLE transaction add COLUMN  cobat numeric(10,2) NOT NULL DEFAULT 0.00;
alter TABLE transaction add COLUMN  soc_hvs numeric(10,2) NOT NULL DEFAULT 0.00;
alter TABLE transaction add COLUMN  soh_hvs numeric(10,2) NOT NULL DEFAULT 0.00;
alter TABLE transaction add COLUMN  mppt_power numeric(10,2) NOT NULL DEFAULT 0.00;
alter TABLE transaction add COLUMN  latitude char(12) NOT NULL DEFAULT 0;
alter TABLE transaction add COLUMN  longitude char(12) NOT NULL DEFAULT 0;