// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Int8,
        created_at -> Int8,
        updated_at -> Int8,
        wx_openid -> Varchar,
        editor -> Int8,
        nick_name -> Varchar,
        password -> Varchar,
        mobile -> Varchar,
        avatar -> Bytea,
        desc -> Text,
    }
}
