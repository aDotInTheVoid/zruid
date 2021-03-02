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
pub struct InvalidMessageError {
    #[serde(rename = "result")]
    pub result: Option<serde_json::Value>,
    #[serde(rename = "msg")]
    pub msg: Option<serde_json::Value>,
    /// The raw content of the message. 
    #[serde(rename = "raw_content", skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
}

impl InvalidMessageError {
    pub fn new(result: Option<serde_json::Value>, msg: Option<serde_json::Value>) -> InvalidMessageError {
        InvalidMessageError {
            result,
            msg,
            raw_content: None,
        }
    }
}


