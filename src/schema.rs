// @generated automatically by Diesel CLI.

diesel::table! {
    table_1 (id) {
        id -> Int4,
        confirm1 -> Bool,
        confirm2 -> Bool,
    }
}

diesel::table! {
    table_2 (id) {
        id -> Int4,
        confirm1 -> Bool,
        confirm2 -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    table_1,
    table_2,
);
