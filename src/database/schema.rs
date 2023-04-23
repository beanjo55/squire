// @generated automatically by Diesel CLI.

diesel::table! {
    counters (id) {
        id -> Uuid,
        name -> Text,
        author -> Int8,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        last_active -> Timestamptz,
        guild_ids -> Nullable<Array<Nullable<Int8>>>,
    }
}

diesel::table! {
    counters_entries (id) {
        id -> Uuid,
        source_counter_id -> Uuid,
        source_guild_id -> Int8,
        user_id -> Int8,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    ha_broadcast_channels (id) {
        id -> Uuid,
        webhook_id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        last_active -> Timestamptz,
    }
}

diesel::table! {
    ha_webhooks (id) {
        id -> Uuid,
        url -> Text,
        secret -> Nullable<Text>,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    highlights (user_id, guild_id, phrase) {
        user_id -> Int8,
        guild_id -> Int8,
        phrase -> Text,
        created_at -> Timestamptz,
        last_used_at -> Nullable<Timestamptz>,
        snooze_until -> Nullable<Timestamp>,
        ignored_channels -> Nullable<Array<Nullable<Int8>>>,
        ignored_users -> Nullable<Array<Nullable<Int8>>>,
    }
}

diesel::joinable!(counters_entries -> counters (source_counter_id));
diesel::joinable!(ha_broadcast_channels -> ha_webhooks (webhook_id));

diesel::allow_tables_to_appear_in_same_query!(
    counters,
    counters_entries,
    ha_broadcast_channels,
    ha_webhooks,
    highlights,
);
