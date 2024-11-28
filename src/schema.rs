// @generated automatically by Diesel CLI.

diesel::table! {
    contents (content_id) {
        content_id -> Int4,
        #[max_length = 160]
        content_title -> Varchar,
        #[max_length = 160]
        content_link -> Nullable<Varchar>,
        content_short -> Int4,
        content_number -> Int4,
        content_sub -> Nullable<Int4>,
        content_parrent -> Nullable<Int4>,
    }
}
