CREATE TABLE IF NOT EXISTS admin_users (
    username VARCHAR(100) NOT NULL UNIQUE,
	email VARCHAR(100) UNIQUE DEFAULT NULL,
	email_verified BOOLEAN DEFAULT NULL,
    secret varchar(50) NOT NULL UNIQUE,
	password TEXT NOT NULL,
	ID SERIAL PRIMARY KEY NOT NULL
);
