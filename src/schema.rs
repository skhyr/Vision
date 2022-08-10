table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        surname -> Varchar,
        access_code -> Int4,
        accounting_day -> Int4,
    }
}
