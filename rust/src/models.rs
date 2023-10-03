use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct InsertableUser<'a> {
    pub id: &'a str,
    pub email: &'a str,
}
