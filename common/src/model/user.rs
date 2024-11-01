
use diesel::{pg::Pg, prelude::*, sql_types::{Bytea, Json, Record}};


#[derive(Queryable, Selectable,Debug, Clone)]
#[diesel(table_name = super::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub wx_openid: String,
    pub editor: i64,
    pub nick_name: String,
    pub password: String,
    pub mobile: String,
    pub avatar: Vec<u8>,
    pub desc: String,
}
