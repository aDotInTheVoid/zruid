/*
 * Zulip REST API
 *
 * Powerful open source group chat 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bot {
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<serde_json::Value>,
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<serde_json::Value>,
    #[serde(rename = "api_key", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<serde_json::Value>,
    #[serde(rename = "default_sending_stream", skip_serializing_if = "Option::is_none")]
    pub default_sending_stream: Option<serde_json::Value>,
    #[serde(rename = "default_events_register_stream", skip_serializing_if = "Option::is_none")]
    pub default_events_register_stream: Option<serde_json::Value>,
    #[serde(rename = "default_all_public_streams", skip_serializing_if = "Option::is_none")]
    pub default_all_public_streams: Option<serde_json::Value>,
    #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<serde_json::Value>,
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<serde_json::Value>,
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<serde_json::Value>,
    /// The email of the bot. 
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// An integer describing the type of bot: * `1` for a `Generic` bot. * `2` for an `Incoming webhook` bot. * `3` for an `Outgoing webhook` bot. * `4` for an `Embedded` bot. 
    #[serde(rename = "bot_type", skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<i32>,
    /// A boolean describing whether the user account has been deactivated. 
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl Bot {
    pub fn new() -> Bot {
        Bot {
            user_id: None,
            full_name: None,
            api_key: None,
            default_sending_stream: None,
            default_events_register_stream: None,
            default_all_public_streams: None,
            avatar_url: None,
            owner_id: None,
            services: None,
            email: None,
            bot_type: None,
            is_active: None,
        }
    }
}


