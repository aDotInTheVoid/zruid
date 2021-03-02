# BasicStreamBase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**stream_id** | Option<**i32**> | The unique ID of the stream.  | [optional]
**name** | Option<**String**> | The name of the stream.  | [optional]
**description** | Option<**String**> | The short description of the stream in text/markdown format, intended to be used to prepopulate UI for editing a stream's description.  | [optional]
**date_created** | Option<**i32**> | The UNIX timestamp for when the stream was created, in UTC seconds.  **Changes**: New in Zulip 4.0 (feature level 30).  | [optional]
**invite_only** | Option<**bool**> | Specifies whether the stream is private or not. Only people who have been invited can access a private stream.  | [optional]
**rendered_description** | Option<**String**> | The short description of the stream rendered as HTML, intended to be used when displaying the stream description in a UI.  One should use the standard Zulip rendered_markdown CSS when displaying this content so that emoji, LaTeX, and other syntax work correctly.  And any client-side security logic for user-generated message content should be applied when displaying this HTML as though it were the body of a Zulip message.  | [optional]
**is_web_public** | Option<**bool**> | Whether the stream has been configured to allow unauthenticated access to its message history from the web.  | [optional]
**stream_post_policy** | Option<**i32**> | Policy for which users can post messages to the stream.  * 1 => Any user can post. * 2 => Only administrators can post. * 3 => Only new members can post.  **Changes**: New in Zulip 3.0, replacing the previous `is_announcement_only` boolean.  | [optional]
**message_retention_days** | Option<**i32**> | Number of days that messages sent to this stream will be stored before being automatically deleted by the [message retention policy](/help/message-retention-policy).  There are two special values:  * `null`, the default, means the stream will inherit the organization   level setting. * `-1` encodes retaining messages in this stream forever.  **Changes**: New in Zulip 3.0 (feature level 17).  | [optional]
**history_public_to_subscribers** | Option<**bool**> | Whether the history of the stream is public to its subscribers.  Currently always true for public streams (i.e. invite_only=False implies history_public_to_subscribers=True), but clients should not make that assumption, as we may change that behavior in the future.  | [optional]
**first_message_id** | Option<**i32**> | The id of the first message in the stream.  Intended to help clients determine whether they need to display UI like the \"more topics\" widget that would suggest the stream has older history that can be accessed.  Null is used for streams with no message history.  | [optional]
**is_announcement_only** | Option<**bool**> | Whether the given stream is announcement only or not.  **Changes**: Deprecated in Zulip 3.0 (feature level 1), use `stream_post_policy` instead.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


