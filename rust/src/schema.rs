// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        #[max_length = 255]
        gender -> Nullable<Varchar>,
        #[max_length = 255]
        education -> Nullable<Varchar>,
        valid -> Nullable<Bool>,
        birth_date -> Nullable<Date>,
        created_at -> Date,
        updated_at -> Date,
    }
}
