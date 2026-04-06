CREATE TABLE users (
	id serial PRIMARY KEY,
	username varchar(255) UNIQUE NOT NULL,
	password_hash varchar(255) NOT NULL,
	created_at timestamp DEFAULT current_timestamp
);

CREATE TABLE products(
	id serial PRIMARY KEY,
	name varchar(255) NOT NULL,
	description text,
	price decimal(10,2) NOT NULL,
	category varchar(50) NOT NULL CHECK (category IN ('servico', 'produto')),
	image_url varchar(500),
	created_at timestamp DEFAULT current_timestamp,
	updated_at timestamp DEFAULT current_timestamp
);

CREATE INDEX idx_products_category ON products(category);

CREATE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN 
	NEW.updated_at = current_timestamp;
	RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER update_products_updated_at
	BEFORE UPDATE ON products
	FOR EACH ROW
	EXECUTE FUNCTION update_updated_at_column();

ALTER TABLE users
ALTER COLUMN created_at SET NOT NULL,  
ALTER COLUMN created_at SET DEFAULT NOW();

ALTER table products RENAME TO items;

ALTER TABLE items
ADD COLUMN stock integer DEFAULT 0;

ALTER TABLE items
ALTER COLUMN price TYPE numeric;

CREATE TABLE users (
	id serial PRIMARY KEY,
	username text NOT NULL,
	email text UNIQUE NOT NULL,
	password_hash text NOT NULL,
	created_at timestamp DEFAULT now()
);

CREATE TABLE items (
	id serial PRIMARY KEY,
	nome text NOT NULL,
	descricao text NOT NULL,
	preco numeric(10, 2) NOT NULL,
	image_url text,
	tipo text DEFAULT 'servico' CHECK (tipo IN ('servico', 'produto'))	
);

ALTER TABLE users
ADD COLUMN updated_at timestamp DEFAULT now();

ALTER TABLE items
ADD COLUMN updated_at timestamp DEFAULT now(),
ADD COLUMN updated_by integer REFERENCES users(id); -- Quem alterou

CREATE TABLE items_logs (
	id serial PRIMARY KEY,
	item_id integer REFERENCES items(id),
	user_id integer REFERENCES users(id),
	acao text NOT NULL, -- 'create', 'update', 'delete'
	data_alteracao timestamp NOT NULL,
	created_at timestamp DEFAULT now()
);

ALTER TABLE items ADD COLUMN link_pagamento TEXT;
