CREATE SCHEMA users_schema AUTHORIZATION users_db_admin;
GRANT USAGE ON SCHEMA users_schema TO rw_user;

CREATE TABLE users_schema.users (
	id int8 NOT NULL GENERATED ALWAYS AS IDENTITY,
	"name" varchar(255) NOT NULL UNIQUE
);

GRANT SELECT,INSERT, UPDATE, DELETE ON users_schema.users TO rw_user;