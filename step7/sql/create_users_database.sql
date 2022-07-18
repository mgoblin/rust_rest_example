CREATE DATABASE users;

CREATE ROLE users_db_admin 
	LOGIN
	CREATEROLE
	NOINHERIT
	CONNECTION LIMIT 10
	PASSWORD '1qaz2wsx';
	
GRANT ALL ON DATABASE users TO users_db_admin;

create role rw_user
  LOGIN
  NOINHERIT
  connection limit 100
  password '123qweasd';

 GRANT CONNECT ON DATABASE users TO rw_user;

