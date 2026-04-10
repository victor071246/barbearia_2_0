CREATE TABLE users (
	id serial PRIMARY KEY,
	username text NOT NULL,
	email text UNIQUE NOT NULL,
	password_hash text NOT NULL,
	created_at timestamp DEFAULT now(),
	updated_at timestamp DEFAULT now()
);

CREATE TABLE items (
	id serial PRIMARY KEY,
	nome text NOT NULL,
	descricao text NOT NULL,
	preco numeric(10, 2) NOT NULL,
	image_url text,
	tipo text DEFAULT 'servico' CHECK (tipo IN ('servico', 'produto')),
	estoque_atual integer DEFAULT 0,
	estoque_minimo integer DEFAULT 0,
	link_pagamento text,
	updated_at timestamp DEFAULT now(),
	updated_by integer REFERENCES users(id)
);

CREATE TABLE items_logs (
	id serial PRIMARY KEY,
	item_id integer REFERENCES items(id),
	user_id integer REFERENCES users(id),
	acao text NOT NULL,
	data_alteracao timestamp NOT NULL,
	estoque_anterior integer DEFAULT 0,
	estoque_definido integer DEFAULT 0,
	created_at timestamp DEFAULT now()
);

CREATE INDEX idx_items_tipo ON items(tipo);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
	NEW.updated_at = current_timestamp;
	RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_items_updated_at
BEFORE UPDATE ON items
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
