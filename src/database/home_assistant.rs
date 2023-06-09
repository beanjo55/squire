use crate::database::schema::{ha_broadcast_channels, ha_webhooks};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = ha_webhooks)]
pub struct HAWebhook {
    pub id: Uuid,
    pub url: String,
    pub secret: Option<String>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = ha_webhooks)]
pub struct NewHAWebhook<'a> {
    pub url: &'a str,
    pub secret: Option<&'a str>,
    pub name: &'a str,
    pub description: Option<&'a str>,
}

#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = ha_broadcast_channels)]
pub struct HABroadcastChannel {
    pub id: Uuid,
    pub webhook_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_active: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = ha_broadcast_channels)]
pub struct NewHABroadcastChannel<'a> {
    pub webhook_id: Uuid,
    pub name: &'a str,
    pub description: Option<&'a str>,
}
