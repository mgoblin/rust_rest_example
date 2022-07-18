revoke all on table users_schema.users from rw_user;
revoke all on schema users_schema from rw_user;

drop table users_schema.users;

drop schema users_schema;
