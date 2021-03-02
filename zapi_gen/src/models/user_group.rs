/*
 * Zulip REST API
 *
 * Powerful open source group chat 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UserGroup : Object containing the user group's attributes. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGroup {
    /// The name of the user group. 
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the user group. 
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Array containing the id of the users who are members of this user group. 
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<i32>>,
    /// The ID of the user group. 
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl UserGroup {
    /// Object containing the user group's attributes. 
    pub fn new() -> UserGroup {
        UserGroup {
            name: None,
            description: None,
            members: None,
            id: None,
        }
    }
}


