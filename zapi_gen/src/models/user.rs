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
pub struct User {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<serde_json::Value>,
    #[serde(rename = "is_bot", skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<serde_json::Value>,
    #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<serde_json::Value>,
    #[serde(rename = "avatar_version", skip_serializing_if = "Option::is_none")]
    pub avatar_version: Option<serde_json::Value>,
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<serde_json::Value>,
    #[serde(rename = "is_admin", skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<serde_json::Value>,
    #[serde(rename = "is_owner", skip_serializing_if = "Option::is_none")]
    pub is_owner: Option<serde_json::Value>,
    #[serde(rename = "bot_type", skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<serde_json::Value>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<serde_json::Value>,
    #[serde(rename = "bot_owner_id", skip_serializing_if = "Option::is_none")]
    pub bot_owner_id: Option<serde_json::Value>,
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<serde_json::Value>,
    #[serde(rename = "is_guest", skip_serializing_if = "Option::is_none")]
    pub is_guest: Option<serde_json::Value>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<serde_json::Value>,
    #[serde(rename = "date_joined", skip_serializing_if = "Option::is_none")]
    pub date_joined: Option<serde_json::Value>,
    #[serde(rename = "delivery_email", skip_serializing_if = "Option::is_none")]
    pub delivery_email: Option<serde_json::Value>,
    #[serde(rename = "profile_data", skip_serializing_if = "Option::is_none")]
    pub profile_data: Option<serde_json::Value>,
}

impl User {
    pub fn new() -> User {
        User {
            email: None,
            is_bot: None,
            avatar_url: None,
            avatar_version: None,
            full_name: None,
            is_admin: None,
            is_owner: None,
            bot_type: None,
            user_id: None,
            bot_owner_id: None,
            is_active: None,
            is_guest: None,
            timezone: None,
            date_joined: None,
            delivery_email: None,
            profile_data: None,
        }
    }
}


