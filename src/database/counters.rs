use crate::database::schema::{counters, counters_entries};
use chrono::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
pub struct Counter {
    pub id: Uuid,
    pub name: String,
    pub author: i64,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_active: NaiveDateTime,
    pub guild_ids: Vec<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = counters)]
pub struct NewCounter<'a> {
    pub name: &'a str,
    pub author: i64,
    pub description: Option<&'a str>,
    pub guild_ids: &'a Vec<i64>,
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
pub fn get_counter_by_name(
    conn: &mut PgConnection,
    counter_name: &str,
    guild_id: &u64,
) -> Result<Option<Counter>, diesel::result::Error> {
    use crate::database::schema::counters::dsl::*;
    counters
        .filter(name.eq(counter_name))
        .filter(guild_ids.contains(vec![*guild_id as i64]))
        .limit(1)
        .first::<Counter>(conn)
        .optional()
}

pub fn get_counter_by_id(
    conn: &mut PgConnection,
    counter_id: &str,
) -> Result<Option<Counter>, diesel::result::Error> {
    use crate::database::schema::counters::dsl::*;
    counters
        .find(Uuid::parse_str(counter_id).unwrap())
        .limit(1)
        .first::<Counter>(conn)
        .optional()
}
