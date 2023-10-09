use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QueryableUser {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub education: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub valid: Option<bool>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct InsertableUser {
    pub id: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub education: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub valid: Option<bool>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

#[derive(AsChangeset, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct ChangesetUser {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub education: Option<String>,
    // pub birth_date: Option<NaiveDate>,
    // pub valid: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewUserProps {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub education: String,
    // pub birth_date: String,
}
