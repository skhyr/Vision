table! {
    organizations (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    transitions (id) {
        id -> Uuid,
        user_id -> Uuid,
        date -> Date,
        fraction -> Float8,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        surname -> Varchar,
        access_code -> Int4,
        accounting_day -> Int4,
        organization_id -> Uuid,
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

joinable!(transitions -> users (user_id));
joinable!(users -> organizations (organization_id));
joinable!(vacations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    organizations,
    transitions,
    users,
    vacations,
);
