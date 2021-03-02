# BasicBotBase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | The user id of the bot.  | [optional]
**full_name** | Option<**String**> | The full name of the bot.  | [optional]
**api_key** | Option<**String**> | The API key of the bot which it uses to make API requests.  | [optional]
**default_sending_stream** | Option<**String**> | The default sending stream of the bot. Null if the bot doesn't have a default sending stream.  | [optional]
**default_events_register_stream** | Option<**String**> | The default stream for which the bot receives events/register data. Null if the bot doesn't have such a default stream.  | [optional]
**default_all_public_streams** | Option<**bool**> | Whether the bot can send messages to all streams by default.  | [optional]
**avatar_url** | Option<**String**> | The URL of the bot's avatar.  | [optional]
**owner_id** | Option<**i32**> | The user id of the bot's owner.  Null if the bot has no owner.  | [optional]
**services** | Option<[**Vec<crate::models::OneOfobjectobject>**](oneOf<object,object>.md)> | The \"Services\" array contains extra configuration fields only relevant for Outgoing webhook bots and Embedded bots.  It is always a single-element array.  We consider this part of the Zulip API to be unstable; it is used only for UI elements for administering bots and is likely to change.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


