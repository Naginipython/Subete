// @generated automatically by Diesel CLI.

diesel::table! {
    chapters (id) {
        id -> Text,
        manga_id -> Text,
        number -> Float,
        title -> Text,
        page -> Integer,
        completed -> Bool,
    }
}

diesel::table! {
    library (id) {
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
    plugins (id) {
        id -> Text,
        search_url -> Text,
        search -> Text,
        chapters_url -> Text,
        get_chapters -> Text,
        pages_url -> Text,
        get_pages -> Text,
    }
}

diesel::joinable!(chapters -> library (manga_id));

diesel::allow_tables_to_appear_in_same_query!(
    chapters,
    library,
    plugins,
);
