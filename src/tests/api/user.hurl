# Test Login API with Language Header
POST http://localhost:8000/login
Content-Type: application/json
Accept-Language: de

{
    "email": "h1@gmail.c",
    "password": "Securepsword123"
}

# Test Logout API with Language Header
DELETE http://localhost:8000/logout
Accept-Language: de

# Create a New User with Language Header
POST http://localhost:8000/users/new
Content-Type: application/json
Accept-Language: de

{
  "name": "user",
  "email": "h1@gmail.com",
  "password": "Securepassword123.",
  "subscription_plan": "Free"
}

# Get All Users with Language Header
GET http://localhost:8000/users/all
Accept-Language: de

# Get User with Language Header
GET http://localhost:8000/users/h1@gmail.om
Accept-Language: id

# Delete User with Language Header
DELETE http://localhost:8000/users/h1@gmail.com
Accept-Language: de

# Update User with Language Header
PUT http://localhost:8000/users/h1@gmail.com
Content-Type: application/json
Accept-Language: jp

{
  "name": "updatedUser1",
  "email": "h1@gmail.com",
  "subscription_plan": "Pro"
}
