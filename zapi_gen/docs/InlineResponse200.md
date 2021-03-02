# InlineResponse200

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bot_email** | Option<**String**> | Email of the bot user.  | [optional]
**data** | Option<**String**> | The message content, in raw Markdown format (not rendered to HTML).  | [optional]
**trigger** | Option<**String**> | What aspect of the message triggered the outgoing webhook notification. Possible values include `private_message` and `mention`.  | [optional]
**token** | Option<**String**> | A string of alphanumeric characters that can be used to authenticate the webhook request (each bot user uses a fixed token). You can get the token used by a given outgoing webhook bot in the `zuliprc` file downloaded when creating the bot.  | [optional]
**message** | Option<[**crate::models::MessagesBase**](MessagesBase.md)> | A dict containing details on the message that triggered the outgoing webhook, in the format used by [`GET /messages`](/api/get-messages).  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


