# GetMessagesAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**avatar_url** | Option<[**serde_json::Value**](.md)> |  | [optional]
**client** | Option<[**serde_json::Value**](.md)> |  | [optional]
**content** | Option<[**serde_json::Value**](.md)> |  | [optional]
**content_type** | Option<[**serde_json::Value**](.md)> |  | [optional]
**display_recipient** | Option<[**serde_json::Value**](.md)> |  | [optional]
**id** | Option<[**serde_json::Value**](.md)> |  | [optional]
**is_me_message** | Option<[**serde_json::Value**](.md)> |  | [optional]
**reactions** | Option<[**serde_json::Value**](.md)> |  | [optional]
**recipient_id** | Option<[**serde_json::Value**](.md)> |  | [optional]
**sender_email** | Option<[**serde_json::Value**](.md)> |  | [optional]
**sender_full_name** | Option<[**serde_json::Value**](.md)> |  | [optional]
**sender_id** | Option<[**serde_json::Value**](.md)> |  | [optional]
**sender_realm_str** | Option<[**serde_json::Value**](.md)> |  | [optional]
**stream_id** | Option<[**serde_json::Value**](.md)> |  | [optional]
**subject** | Option<[**serde_json::Value**](.md)> |  | [optional]
**topic_links** | Option<[**serde_json::Value**](.md)> |  | [optional]
**submessages** | Option<[**serde_json::Value**](.md)> |  | [optional]
**timestamp** | Option<[**serde_json::Value**](.md)> |  | [optional]
**_type** | Option<[**serde_json::Value**](.md)> |  | [optional]
**flags** | Option<**Vec<String>**> | The user's [message flags][message-flags] for the message.  | [optional]
**last_edit_timestamp** | Option<**i32**> | The UNIX timestamp for when the message was last edited, in UTC seconds.  | [optional]
**match_content** | Option<**String**> | Only present if keyword search was included among the narrow parameters. HTML content of a queried message that matches the narrow, with `<span class=\"highlight\">` elements wrapping the matches for the search keywords.  | [optional]
**match_subject** | Option<**String**> | Only present if keyword search was included among the narrow parameters. HTML-escaped topic of a queried message that matches the narrow, with `<span class=\"highlight\">` elements wrapping the matches for the search keywords.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


