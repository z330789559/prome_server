-- Add migration script here

CREATE TABLE node.notes(
    title varchar(255) NOT NULL,
    content text NOT NULL,
    category varchar(100),
    published boolean DEFAULT false,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    id SERIAL NOT NULL
);
CREATE UNIQUE INDEX notes_title_key ON node.notes USING btree ("title");