# Rust Rest service example using postgres, actix-web, rbatis and refinery

This is my Rust language learning project.

As a Java Spring Boot backend developer i decide to experiment with simple rest/json CRUD webservice on top of postgres DB.

## Rest service functionality
User has unique id and name.

WebService endpoints

- GET /users?page_no=0&page_size=10 return paginated json list of users ordered by name. page_no and page_size query parameters are optional.
- GET /users/{id} - return user with id as json or http 404 if user not found
- POST /user with http request body {name: <user name>} create user then return http 200 and user json
- POST /user/{id} with request body {name: <user name>} update user then return http 200 and user json or 404 if user not found