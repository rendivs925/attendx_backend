POST http://localhost:8000/auth/login
Content-Type: application/json
Accept-Language: ja

{
  "email": "h1@gmail.com",
  "password": "Securepassword123."
}

DELETE http://localhost:8000/auth/logout
Accept-Language: de

POST http://localhost:8000/auth/register
Content-Type: application/json
Accept-Language: ja

{
  "name": "user",
  "email": "h1@gmail.com",
  "password": "Securepassword123.",
  "subscription_plan": "Free"
}

GET http://localhost:8000/users/all
Accept-Language: de

GET http://localhost:8000/users/h1@gmail.om
Accept-Language: id

DELETE http://localhost:8000/users/h1@gmail.com
Accept-Language: de

PUT http://localhost:8000/users/h1@gmail.com
Content-Type: application/json
Accept-Language: ja

{
  "name": "updatedUser1",
  "email": "h1@gmail.com",
  "subscription_plan": "Pro"
}
