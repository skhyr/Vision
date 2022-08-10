table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        surname -> Varchar,
        access_code -> Int4,
        accounting_day -> Int4,
    }
}

table! {
    vacations (id) {
        id -> Uuid,
        user_id -> Uuid,
        start_date -> Date,
        end_date -> Date,
        title -> Varchar,
    }
}

joinable!(vacations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    vacations,
);
