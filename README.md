<h1>Spider Devops Inductions - Task 2 </h1>

<h2><b>Instructions:</b></h2>
1. Fork the given repo and set the your repo to <b>private</b>.<br />
2. Setup the servers using the instructions given below<br />
3. Dockerize the given application using <b>Docker</b> and <b>Docker-compose</b><br />
<br/>
<hr/>
<h2><b>Tech Stack:</b></h2>
Language: <i>Rust</i><br />
Framework: <i>Actix-Web</i><br />
ORM: <i>Diesel</i><br />
Database: <i>PosgreSQL</i><br />
Frontend: <i>React</i><br />
Compilers: <i>Rust v1.7.0 and node v18.0.0</i><br />
<br/>
<hr/>

<b>How to Setup Server[Linux installation]:</b><br />
- ```cd Backend```<br />
- ```sudo apt install libpq-dev```
- ```cargo install diesel_cli --no-default-features --features postgres```
- CREATE DATABASE rust_server [in psql CLI];
- Add ENV variables for DB connection<br />
- ```diesel setup``` [Generates the tables]<br />
- ```cargo run``` [server runs]

<i>Any problems with setting up server refer to official cargo and diesel documents</i>

<br/>
<hr/>

<b>How to Setup Frontend React Server:</b><br />
- ```cd Frontend```<br />
- ```npm i``` [install dependencies]<br />
- ```npm start``` [start react server]<br />
