pub mod counters;
pub mod highlights;
pub mod home_assistant;
pub mod schema;

use diesel::prelude::*;
use uuid::Uuid;

use self::counters::Counter;

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
