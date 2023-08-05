**Show User**
----
  Returns json data about a single user.

* **URL**
  /getUser/:email

* **Method:**
  `GET`
  
*  **URL Params**
   **Required:**
   `email=[string]`

* **Data Params**
  None

* **Success Response:**
  * **Code:** 200 <br />
    **Content:** [{"id": 1,"user_name": "rithvik","user_email": "rithvik@gmail.com","user_password":"password"}]

* **Error Response:**

 * **Code:** 500 <br />
    No users found with email: rithvik.ravila@gmail.com

**Show All Users**
----
  Returns json data about all users.

* **URL**
  /getUser

* **Method:**
  `GET`
  
*  **URL Params**
   None

* **Data Params**
   None

* **Success Response:**
  * **Code:** 200 <br />
    **Content:** [{"id": 1,"user_name": "rithvik","user_email": "rithvik@gmail.com","user_password":"password"}]

* **Error Response:**

 * **Code:** 500 <br />
    No users found with email: rithvik.ravila@gmail.com

**Create User**
----
  Creates a single user.

* **URL**
  /createUser

* **Method:**
  `POST`
  
*  **URL Params**
   None

* **Data Params**
   **JSON**
   {"user_name": "2rithvik2","user_email": "ravilla@gmail.com","user_password": "password2"}

* **Success Response:**
  * **Code:** 200 <br />
    **Content:** [{"id": 2,"user_name": "2rithvik2","user_email": "ravilla@gmail.com","user_password":"password"}]

**login User**
----
  Login.

* **URL**
  /loginUser

* **Method:**
  `POST`
  
*  **URL Params**
   None

* **Data Params**
   **JSON**
   {"user_email": "ravilla@gmail.com","user_password": "password2"}

* **Success Response:**
  * **Code:** 200 <br />
    **Content:** {"message": "Login successful"} <br />

* **Error Response:**

 * **Code:** 500 <br />
    No users found with email: rithvik.ravila@gmail.com

  * **Code:** 500 <br />
    Password Incorrect

 

