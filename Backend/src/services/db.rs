use diesel::{pg::PgConnection};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use crate::DbError;
use crate::models::models::{NewUser,User, CreateUser, UpdateUser,Login,Response};
use dotenvy::dotenv;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    // let database_url = String::from("postgres://postgres:Try2read@localhost:5432/rust_server");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    return pool;
}

pub fn create_user(conn: &mut PgConnection, info: &mut CreateUser) -> Result<User,DbError> {
    use crate::schema::users;
    let new_user = NewUser { 
        user_name: &info.user_name
        , user_email: &info.user_email
        ,user_password: &info.user_password 
    };
    let create = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)?;
    Ok(create)
}

pub fn get_all_users(conn: &mut PgConnection) -> Result<Vec<User>,DbError> {
    use crate::schema::users::dsl::*;
    let items = users.load::<User>(conn)?;
    Ok(items)
}

pub fn get_users(conn: &mut PgConnection,email: &str) -> Result<User,DbError> {
    use crate::schema::users::dsl::*;

    let user_result = users.filter(user_email.eq(email)).first::<User>(conn);
    match user_result {
        Ok(user) => Ok(user),
        Err(diesel::result::Error::NotFound) => {
            let error_message = format!("No users found with email: {}", email);
            // let custom_error = Response { message: error_message };
            let test: DbError = String::from(error_message).into();
            Err(test)
        }
        Err(err) => Err(Box::new(err)),
    }
}

pub fn update_user(conn: &mut PgConnection,email: &str,update_details: &UpdateUser) -> Result<User,DbError> {
    use crate::schema::users::dsl::*;
    let person = get_users(conn,email)?;
  
  let mut final_password = person.user_password;
  match &update_details.user_password {
    Some(x) => final_password = x.to_string(),
    None => {},
  }

  let mut final_name = person.user_name;
  match &update_details.user_name {
    Some(x) => final_name = x.to_string(),
    None => {},
  }

    let item = diesel::update(users.filter(id.eq(person.id)))
    .set((user_name.eq(final_name),user_password.eq(final_password)))
    .get_result::<User>(conn)?;
    Ok(item)
}

pub fn delete_user(conn: &mut PgConnection,email: &str) -> Result<Response,DbError> {
    use crate::schema::users::dsl::*;
    let person = get_users(conn,email)?;
    diesel::delete(users.find(person.id)).execute(conn)?;
    let message = Response {message: String::from("Delete Successful")};
    Ok(message)
}

pub fn check_login(conn: &mut PgConnection,login: & mut Login) -> Result<Response,DbError> {

    let person = get_users(conn,&login.user_email)?;
    
    if person.user_password == login.user_password {
        let final_message = Response {message: String::from("Login successful")};
        return Ok(final_message);
    } else {
        let test: DbError = String::from("Password Incorrect").into();
        return Err(test);
    }
}