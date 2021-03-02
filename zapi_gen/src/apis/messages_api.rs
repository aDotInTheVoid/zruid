/*
 * Zulip REST API
 *
 * Powerful open source group chat 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `add_reaction`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddReactionError {
    Status400(crate::models::CodedError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `check_messages_match_narrow`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckMessagesMatchNarrowError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteMessageError {
    Status400(crate::models::OneOfInvalidMessageErrorobject),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_file_temporary_url`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFileTemporaryUrlError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_message_history`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMessageHistoryError {
    Status400(crate::models::InvalidMessageError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_messages`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMessagesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_raw_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRawMessageError {
    Status400(crate::models::InvalidMessageError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `mark_all_as_read`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkAllAsReadError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `mark_stream_as_read`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkStreamAsReadError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `mark_topic_as_read`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkTopicAsReadError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `remove_reaction`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveReactionError {
    Status400(crate::models::CodedError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `render_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RenderMessageError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `send_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendMessageError {
    Status400(crate::models::OneOfNonExistingStreamErrorobject),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMessageError {
    Status400(crate::models::CodedError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `update_message_flags`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMessageFlagsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `upload_file`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadFileError {
    UnknownValue(serde_json::Value),
}


/// Add an [emoji reaction](/help/emoji-reactions) to a message.  `POST {{ api_url }}/v1/messages/{message_id}/reactions` 
pub async fn add_reaction(configuration: &configuration::Configuration, message_id: i32, emoji_name: &str, emoji_code: Option<&str>, reaction_type: Option<&str>) -> Result<crate::models::JsonSuccess, Error<AddReactionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/messages/{message_id}/reactions", configuration.base_path, message_id=message_id);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("emoji_name", &emoji_name.to_string())]);
    if let Some(ref local_var_str) = emoji_code {
        local_var_req_builder = local_var_req_builder.query(&[("emoji_code", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = reaction_type {
        local_var_req_builder = local_var_req_builder.query(&[("reaction_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddReactionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Check whether a set of messages match a [narrow](/api/construct-narrow).  `GET {{ api_url }}/v1/messages/matches_narrow`  For many common narrows (E.g. a topic), clients can write an efficient client-side check to determine whether a newly arrived message belongs in the view.  This endpoint is designed to allow clients to handle more complex narrows for which the client does not (or in the case of full-text search, cannot) implement this check.  The format of the `match_subject` and `match_content` objects is designed to match those of `GET /messages`, so that a client can splice these fields into a `message` object received from `GET /events` and end up with an extended message object identical to how a `GET /messages` for the current narrow would have returned the message. 
pub async fn check_messages_match_narrow(configuration: &configuration::Configuration, msg_ids: Vec<i32>, narrow: Vec<serde_json::Value>) -> Result<crate::models::JsonSuccessBase, Error<CheckMessagesMatchNarrowError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/messages/matches_narrow", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("msg_ids", &msg_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("narrow", &narrow.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CheckMessagesMatchNarrowError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Permanently delete a message.  `DELETE {{ api_url }}/v1/messages/{msg_id}`  This API corresponds to the [delete a message completely][delete-completely] feature documented in the Zulip Help Center. 
pub async fn delete_message(configuration: &configuration::Configuration, message_id: i32) -> Result<crate::models::JsonSuccess, Error<DeleteMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/messages/{message_id}", configuration.base_path, message_id=message_id);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a temporary URL for access to the file that doesn't require authentication. 
pub async fn get_file_temporary_url(configuration: &configuration::Configuration, realm_id_str: i32, filename: &str) -> Result<crate::models::JsonSuccessBase, Error<GetFileTemporaryUrlError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/user_uploads/{realm_id_str}/{filename}", configuration.base_path, realm_id_str=realm_id_str, filename=crate::apis::urlencode(filename));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFileTemporaryUrlError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch the message edit history of a previously edited message.  `GET {{ api_url }}/v1/messages/{message_id}/history`  Note that edit history may be disabled in some organizations; see the [Zulip Help Center documentation on editing messages][edit-settings].  [edit-settings]: /help/view-a-messages-edit-history 
pub async fn get_message_history(configuration: &configuration::Configuration, message_id: i32) -> Result<crate::models::JsonSuccessBase, Error<GetMessageHistoryError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/messages/{message_id}/history", configuration.base_path, message_id=message_id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetMessageHistoryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch message history from a Zulip server.  `GET {{ api_url }}/v1/messages`  This `GET /api/v1/messages` endpoint is the primary way to fetch message history from a Zulip server.  It is useful both for Zulip clients (e.g. the web, desktop, mobile, and terminal clients) as well as bots, API clients, backup scripts, etc.  By specifying a [narrow filter](/api/construct-narrow), you can use this endpoint to fetch the messages matching any search query that is supported by Zulip's powerful full-text search backend.  When a narrow is not specified, it can be used to fetch a user's message history. (We recommend paginating to 1000 messages at a time.)  In either case, you specify an `anchor` message (or ask the server to calculate the first unread message for you and use that as the anchor), as well as a number of messages before and after the anchor message.  The server returns those messages, sorted by message ID, as well as some metadata that makes it easy for a client to determine whether there are more messages matching the query that were not returned due to the `num_before` and `num_after` limits.  We recommend using `num_before <= 1000` and `num_after <= 1000` to avoid generating very large HTTP responses. A maximum of 5000 messages can be obtained per request; attempting to exceed this will result in an error. 
pub async fn get_messages(configuration: &configuration::Configuration, num_before: i32, num_after: i32, anchor: Option<crate::models::OneOfstringinteger>, narrow: Option<Vec<serde_json::Value>>, client_gravatar: Option<bool>, apply_markdown: Option<bool>, use_first_unread_anchor: Option<bool>) -> Result<crate::models::JsonSuccessBase, Error<GetMessagesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/messages", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = anchor {
        local_var_req_builder = local_var_req_builder.query(&[("anchor", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("num_before", &num_before.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("num_after", &num_after.to_string())]);
    if let Some(ref local_var_str) = narrow {
        local_var_req_builder = local_var_req_builder.query(&[("narrow", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    if let Some(ref local_var_str) = client_gravatar {
        local_var_req_builder = local_var_req_builder.query(&[("client_gravatar", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = apply_markdown {
        local_var_req_builder = local_var_req_builder.query(&[("apply_markdown", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = use_first_unread_anchor {
        local_var_req_builder = local_var_req_builder.query(&[("use_first_unread_anchor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetMessagesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the raw content of a message.  `GET {{ api_url }}/v1/messages/{msg_id}`  This is a rarely-used endpoint relevant for clients that primarily work with HTML-rendered messages but might need to occasionally fetch the message's raw Markdown (e.g. for pre-filling a message-editing UI). 
pub async fn get_raw_message(configuration: &configuration::Configuration, message_id: i32) -> Result<crate::models::JsonSuccessBase, Error<GetRawMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/messages/{message_id}", configuration.base_path, message_id=message_id);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRawMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Marks all of the current user's unread messages as read.  `POST {{ api_url }}/v1/mark_all_as_read` 
pub async fn mark_all_as_read(configuration: &configuration::Configuration, ) -> Result<crate::models::JsonSuccess, Error<MarkAllAsReadError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/mark_all_as_read", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarkAllAsReadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Mark all the unread messages in a stream as read. 
pub async fn mark_stream_as_read(configuration: &configuration::Configuration, stream_id: i32) -> Result<crate::models::JsonSuccess, Error<MarkStreamAsReadError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/mark_stream_as_read", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("stream_id", &stream_id.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarkStreamAsReadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Mark all the unread messages in a topic as read. 
pub async fn mark_topic_as_read(configuration: &configuration::Configuration, stream_id: i32, topic_name: &str) -> Result<crate::models::JsonSuccess, Error<MarkTopicAsReadError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/mark_topic_as_read", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("stream_id", &stream_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("topic_name", &topic_name.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MarkTopicAsReadError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove an [emoji reaction](/help/emoji-reactions) from a message.  `DELETE {{ api_url }}/v1/messages/{message_id}/reactions` 
pub async fn remove_reaction(configuration: &configuration::Configuration, message_id: i32, emoji_name: Option<&str>, emoji_code: Option<&str>, reaction_type: Option<&str>) -> Result<crate::models::JsonSuccess, Error<RemoveReactionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/messages/{message_id}/reactions", configuration.base_path, message_id=message_id);
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = emoji_name {
        local_var_req_builder = local_var_req_builder.query(&[("emoji_name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = emoji_code {
        local_var_req_builder = local_var_req_builder.query(&[("emoji_code", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = reaction_type {
        local_var_req_builder = local_var_req_builder.query(&[("reaction_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RemoveReactionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Render a message to HTML.  `POST {{ api_url }}/v1/messages/render` 
pub async fn render_message(configuration: &configuration::Configuration, content: &str) -> Result<crate::models::JsonSuccessBase, Error<RenderMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/messages/render", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("content", &content.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RenderMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Send a stream or a private message.  `POST {{ api_url }}/v1/messages` 
pub async fn send_message(configuration: &configuration::Configuration, _type: &str, to: Vec<i32>, content: &str, topic: Option<&str>, queue_id: Option<&str>, local_id: Option<&str>) -> Result<crate::models::JsonSuccessBase, Error<SendMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/messages", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("type", &_type.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("to", &to.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("content", &content.to_string())]);
    if let Some(ref local_var_str) = topic {
        local_var_req_builder = local_var_req_builder.query(&[("topic", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = queue_id {
        local_var_req_builder = local_var_req_builder.query(&[("queue_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = local_id {
        local_var_req_builder = local_var_req_builder.query(&[("local_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SendMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Edit/update the content or topic of a message.  `PATCH {{ api_url }}/v1/messages/{msg_id}`  `{msg_id}` in the above URL should be replaced with the ID of the message you wish you update. 
pub async fn update_message(configuration: &configuration::Configuration, message_id: i32, topic: Option<&str>, propagate_mode: Option<&str>, send_notification_to_old_thread: Option<bool>, send_notification_to_new_thread: Option<bool>, content: Option<&str>, stream_id: Option<i32>) -> Result<crate::models::JsonSuccess, Error<UpdateMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/messages/{message_id}", configuration.base_path, message_id=message_id);
    let mut local_var_req_builder = local_var_client.patch(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = topic {
        local_var_req_builder = local_var_req_builder.query(&[("topic", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = propagate_mode {
        local_var_req_builder = local_var_req_builder.query(&[("propagate_mode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = send_notification_to_old_thread {
        local_var_req_builder = local_var_req_builder.query(&[("send_notification_to_old_thread", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = send_notification_to_new_thread {
        local_var_req_builder = local_var_req_builder.query(&[("send_notification_to_new_thread", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = content {
        local_var_req_builder = local_var_req_builder.query(&[("content", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = stream_id {
        local_var_req_builder = local_var_req_builder.query(&[("stream_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Add or remove personal message flags like `read` and `starred` on a collection of message IDs.  `POST {{ api_url }}/v1/messages/flags`  For updating the `read` flag on common collections of messages, see also the [special endpoints for marking message as read in bulk](/api/mark-all-as-read). 
pub async fn update_message_flags(configuration: &configuration::Configuration, messages: Vec<i32>, op: &str, flag: &str) -> Result<crate::models::JsonSuccessBase, Error<UpdateMessageFlagsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/messages/flags", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("messages", &messages.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("op", &op.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("flag", &flag.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateMessageFlagsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Upload a single file and get the corresponding URI.  `POST {{ api_url }}/v1/user_uploads`  Initially, only you will be able to access the link.  To share the uploaded file, you'll need to [send a message][send-message] containing the resulting link.  Users who can already access the link can reshare it with other users by sending additional Zulip messages containing the link.  [uploaded-files]: /help/manage-your-uploaded-files [send-message]: /api/send-message 
pub async fn upload_file(configuration: &configuration::Configuration, filename: Option<std::path::PathBuf>) -> Result<crate::models::JsonSuccessBase, Error<UploadFileError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/user_uploads", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    let mut local_var_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'filename' parameter
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UploadFileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

