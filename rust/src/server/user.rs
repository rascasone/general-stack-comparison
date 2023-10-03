use crate::models::User;
use leptos::{server, ServerFnError};

#[server(GetAllUsers, "/api", "Cbor")]
pub async fn get_all_users() -> Result<Vec<User>, ServerFnError> {
    use crate::database::establish_connection;
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;

    let connection = &mut establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    Ok(results)
}

#[server(CreateNewUser, "/api", "Cbor")]
pub async fn create_new_user(user_email: String) -> Result<String, ServerFnError> {
    use crate::database::establish_connection;
    use crate::models::InsertableUser;
    use crate::schema::users::dsl::*;
    use diesel::insert_into;
    use diesel::prelude::*;
    use uuid::Uuid;

    let user_id = Uuid::new_v4().to_string();
    let connection = &mut establish_connection();

    insert_into(users)
        .values(InsertableUser {
            id: &user_id,
            email: &user_email,
        })
        .execute(connection)
        .expect("Error creating a new user");

    Ok(user_id)
}
