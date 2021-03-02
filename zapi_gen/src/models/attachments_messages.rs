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
pub struct AttachmentsMessages {
    /// Time when the message was sent as a UNIX timestamp multiplied by 1000 (matching the format of getTime() in JavaScript).  **Changes**: Changed in Zulip 2.2 (feature level 22).  This field was previously strangely called `name` and was a floating point number. 
    #[serde(rename = "date_sent", skip_serializing_if = "Option::is_none")]
    pub date_sent: Option<i32>,
    /// The unique message ID.  Messages should always be displayed sorted by ID. 
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl AttachmentsMessages {
    pub fn new() -> AttachmentsMessages {
        AttachmentsMessages {
            date_sent: None,
            id: None,
        }
    }
}


