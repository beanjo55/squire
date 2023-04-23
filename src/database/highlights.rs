use crate::database::schema::highlights;
use chrono::NaiveDateTime;
use diesel::helper_types::Nullable;
use diesel::prelude::*;

#[derive(Queryable)]
#[diesel(table_name = highlights)]
#[diesel(primary_key(user_id, guild_id, phrase))]
pub struct Highlight {
    pub user_id: i64,
    pub guild_id: i64,
    pub phrase: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: Nullable<NaiveDateTime>,
    pub snooze_until: Nullable<NaiveDateTime>,
    pub ignored_channels: Nullable<Vec<i64>>,
    pub ignored_users: Nullable<Vec<i64>>,
}

#[derive(Insertable)]
#[diesel(table_name = highlights)]
pub struct NewHighlight<'a> {
    pub user_id: i64,
    pub guild_id: i64,
    pub phrase: &'a str,
    pub ignored_channels: Option<&'a Vec<i64>>,
    pub ignored_users: Option<&'a Vec<i64>>,
}
