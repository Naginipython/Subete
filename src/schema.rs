diesel::table! {
    manga (id) {
        id -> Text,
        title -> Text,
        img -> Text,
        extension -> Text,
        authors -> Text,
        artists -> Text,
        description -> Text,
    }
}

diesel::table! {
    chapter (id) {
        id -> Text,
        manga_id -> Text,
        number -> Float,
        title -> Text,
        page -> Integer,
        completed -> Bool,
    }
}

diesel::joinable!(chapter -> manga (manga_id));

diesel::allow_tables_to_appear_in_same_query!(
    manga,
    chapter,
);
