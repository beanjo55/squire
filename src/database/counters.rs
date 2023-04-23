use crate::database::schema::{counters, counters_entries};
use chrono::NaiveDateTime;
use diesel::helper_types::Nullable;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable)]
pub struct Counter {
    pub id: Uuid,
    pub name: String,
    pub author: i64,
    pub description: Nullable<String>,
    pub created_at: NaiveDateTime,
    pub last_active: NaiveDateTime,
    pub guild_ids: Nullable<Vec<i64>>,
}

#[derive(Insertable)]
#[diesel(table_name = counters)]
pub struct NewCounter<'a> {
    pub name: &'a str,
    pub author: i64,
    pub description: Option<&'a str>,
    pub guild_ids: Option<&'a Vec<i64>>,
}

#[derive(Queryable, Associations, Identifiable)]
#[diesel(belongs_to(Counter, foreign_key = source_counter_id ))]
#[diesel(table_name = counters_entries)]
pub struct CounterEntry {
    pub id: Uuid,
    pub source_counter_id: Uuid,
    pub source_guild_id: i64,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = counters_entries)]
pub struct NewCounterEntry {
    pub source_counter_id: Uuid,
    pub source_guild_id: i64,
    pub user_id: i64,
}
