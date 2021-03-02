/*
 * Zulip REST API
 *
 * Powerful open source group chat 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserBase : A dictionary containing basic data on a given Zulip user. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserBase {
    /// The Zulip API email address of the user or bot.  If you do not have permission to view the email address of the target user, this will be a fake email address that is usable for the Zulip API but nothing else. 
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// A boolean specifying whether the user is a bot or full account. 
    #[serde(rename = "is_bot", skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<bool>,
    /// URL for the user's avatar.  Will be `null` if the `client_gravatar` query parameter was set to `True` and the user's avatar is hosted by the Gravatar provider (i.e. the user has never uploaded an avatar).  **Changes**: In Zulip 3.0 (feature level 18), if the client has the `user_avatar_url_field_optional` capability, this will be missing at the server's sole discretion. 
    #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Version for the user's avatar.  Used for cache-busting requests for the user's avatar.  Clients generally shouldn't need to use this; most avatar URLs sent by Zulip will already end with `?v={avatar_version}`. 
    #[serde(rename = "avatar_version", skip_serializing_if = "Option::is_none")]
    pub avatar_version: Option<i32>,
    /// Full name of the user or bot, used for all display purposes. 
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// A boolean specifying whether the user is an organization administrator. 
    #[serde(rename = "is_admin", skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    /// A boolean specifying whether the user is an organization owner. If true, is_admin will also be true.  **Changes**: New in Zulip 3.0 (feature level 8). 
    #[serde(rename = "is_owner", skip_serializing_if = "Option::is_none")]
    pub is_owner: Option<bool>,
    /// An integer describing the type of bot: * `null` if the user isn't a bot. * `1` for a `Generic` bot. * `2` for an `Incoming webhook` bot. * `3` for an `Outgoing webhook` bot. * `4` for an `Embedded` bot. 
    #[serde(rename = "bot_type", skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<i32>,
    /// The unique ID of the user. 
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// If the user is a bot (i.e. `is_bot` is `True`), `bot_owner` is the user ID of the bot's owner (usually, whoever created the bot).  Will be null for legacy bots that do not have an owner.  **Changes**: New in Zulip 3.0 (feature level 1).  In previous versions, there was a `bot_owner` field containing the email address of the bot's owner. 
    #[serde(rename = "bot_owner_id", skip_serializing_if = "Option::is_none")]
    pub bot_owner_id: Option<i32>,
    /// A boolean specifying whether the user account has been deactivated. 
    #[serde(rename = "is_active", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// A boolean specifying whether the user is a guest user. 
    #[serde(rename = "is_guest", skip_serializing_if = "Option::is_none")]
    pub is_guest: Option<bool>,
    /// The time zone of the user. 
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The time the user account was created. 
    #[serde(rename = "date_joined", skip_serializing_if = "Option::is_none")]
    pub date_joined: Option<String>,
    /// The user's real email address.  This field is present only if [email address visibility](/help/restrict-visibility-of-email-addresses) is limited and you are an administrator with access to real email addresses under the configured policy. 
    #[serde(rename = "delivery_email", skip_serializing_if = "Option::is_none")]
    pub delivery_email: Option<String>,
    /// A dictionary containing custom profile field data for the user. Each entry maps the integer ID of a custom profile field in the organization to a dictionary containing the user's data for that field.  Generally the data includes just a single `value` key; for those custom profile fields supporting Markdown, a `rendered_value` key will also be present. 
    #[serde(rename = "profile_data", skip_serializing_if = "Option::is_none")]
    pub profile_data: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl UserBase {
    /// A dictionary containing basic data on a given Zulip user. 
    pub fn new() -> UserBase {
        UserBase {
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


