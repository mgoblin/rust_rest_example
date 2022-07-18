REVOKE ALL ON DATABASE users FROM users_db_admin;
DROP ROLE users_db_admin;

REVOKE CONNECT ON DATABASE users FROM rw_user;
drop role rw_user;

drop database users WITH (FORCE);