use leptos::{server, ServerFnError};

use crate::models::{ChangesetUser, NewUserProps, QueryableUser};

#[server(ServerGetUsers, "/api", "Cbor")]
pub async fn get_users() -> Result<Vec<QueryableUser>, ServerFnError> {
    use crate::database::establish_connection;
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;
    use tokio::time::sleep;

    let connection = &mut establish_connection();
    let results = users
        .select(QueryableUser::as_select())
        .load(connection)
        .expect("Error loading users");

    sleep(std::time::Duration::from_secs(1)).await;

    Ok(results)
}

#[server(ServerCreateUser, "/api", "Cbor")]
pub async fn create_user(user: NewUserProps) -> Result<String, ServerFnError> {
    use crate::database::establish_connection;
    use crate::models::InsertableUser;
    use crate::schema::users::dsl::*;
    use chrono::Local;
    use diesel::insert_into;
    use diesel::prelude::*;
    use uuid::Uuid;

    let user_id = Uuid::new_v4().to_string();
    let connection = &mut establish_connection();

    let string_to_option = |value: String| {
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    };

    insert_into(users)
        .values(InsertableUser {
            id: user_id.clone(),
            email: user.email,
            first_name: string_to_option(user.first_name),
            last_name: string_to_option(user.last_name),
            gender: string_to_option(user.gender),
            education: string_to_option(user.education),
            // birth_date: match user.birth_date.is_empty() {
            //     true => None,
            //     false => {
            //         Some(chrono::NaiveDate::parse_from_str(&user.birth_date, "%Y-%m-%d").unwrap())
            //     }
            // },
            birth_date: Some(Local::now().date_naive()),
            valid: Some(false),
            created_at: Local::now().date_naive(),
            updated_at: Local::now().date_naive(),
        })
        .execute(connection)
        .expect("Error creating a new user");

    Ok(user_id)
}

#[server(ServerUpdateUser, "/api", "Cbor")]
pub async fn update_user(user_id: String, user: ChangesetUser) -> Result<(), ServerFnError> {
    use crate::database::establish_connection;
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;
    use diesel::update;

    let connection = &mut establish_connection();

    update(users)
        .filter(id.eq(user_id))
        .set(user)
        .execute(connection)
        .expect("Error updating a user");

    Ok(())
}

#[server(ServerDeleteUser, "/api", "Cbor")]
pub async fn delete_user(user_id: String) -> Result<(), ServerFnError> {
    use crate::database::establish_connection;
    use crate::schema::users::dsl::*;
    use diesel::delete;
    use diesel::prelude::*;

    let connection = &mut establish_connection();

    delete(users.filter(id.eq(user_id)))
        .execute(connection)
        .expect("Error creating a new user");

    Ok(())
}
