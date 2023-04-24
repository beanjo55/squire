use crate::database::schema::highlights;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
#[diesel(table_name = highlights)]
#[diesel(primary_key(user_id, guild_id, phrase))]
pub struct Highlight {
    pub user_id: i64,
    pub guild_id: i64,
    pub phrase: String,
    pub created_at: NaiveDateTime,
    pub last_used_at: Option<NaiveDateTime>,
    pub snooze_until: Option<NaiveDateTime>,
    pub ignored_channels: Option<Vec<i64>>,
    pub ignored_users: Option<Vec<i64>>,
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
