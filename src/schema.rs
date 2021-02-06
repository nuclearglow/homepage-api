table! {
    items (id) {
        id -> Int8,
        list_id -> Int8,
        title -> Varchar,
        amount -> Int4,
    }
}

table! {
    lists (id) {
        id -> Int8,
        title -> Varchar,
        subtitle -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(items, lists,);
