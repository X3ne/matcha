use crate::shared::types::snowflake::Snowflake;

#[derive(Debug, serde::Serialize)]
#[serde(tag = "op", content = "data")]
pub enum GatewayEvent {
    MessageReceived {
        channel_id: Snowflake,
        sender_id: Snowflake,
        sender_username: String,
        sender_avatar: Option<String>,
        content: String,
    },
    LikeReceived {
        user_id: Snowflake,
        username: String,
        avatar: Option<String>,
    },
    ProfileViewed {
        user_id: Snowflake,
        username: String,
        avatar: Option<String>,
    },
    NewMatch {
        user_id: Snowflake,
        username: String,
        avatar: Option<String>,
    },
    MatchRemoved {
        user_id: Snowflake,
        username: String,
        avatar: Option<String>,
    },
    SystemNotification {
        message: String,
    },
}
