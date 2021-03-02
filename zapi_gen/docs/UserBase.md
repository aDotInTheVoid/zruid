# UserBase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> | The Zulip API email address of the user or bot.  If you do not have permission to view the email address of the target user, this will be a fake email address that is usable for the Zulip API but nothing else.  | [optional]
**is_bot** | Option<**bool**> | A boolean specifying whether the user is a bot or full account.  | [optional]
**avatar_url** | Option<**String**> | URL for the user's avatar.  Will be `null` if the `client_gravatar` query parameter was set to `True` and the user's avatar is hosted by the Gravatar provider (i.e. the user has never uploaded an avatar).  **Changes**: In Zulip 3.0 (feature level 18), if the client has the `user_avatar_url_field_optional` capability, this will be missing at the server's sole discretion.  | [optional]
**avatar_version** | Option<**i32**> | Version for the user's avatar.  Used for cache-busting requests for the user's avatar.  Clients generally shouldn't need to use this; most avatar URLs sent by Zulip will already end with `?v={avatar_version}`.  | [optional]
**full_name** | Option<**String**> | Full name of the user or bot, used for all display purposes.  | [optional]
**is_admin** | Option<**bool**> | A boolean specifying whether the user is an organization administrator.  | [optional]
**is_owner** | Option<**bool**> | A boolean specifying whether the user is an organization owner. If true, is_admin will also be true.  **Changes**: New in Zulip 3.0 (feature level 8).  | [optional]
**bot_type** | Option<**i32**> | An integer describing the type of bot: * `null` if the user isn't a bot. * `1` for a `Generic` bot. * `2` for an `Incoming webhook` bot. * `3` for an `Outgoing webhook` bot. * `4` for an `Embedded` bot.  | [optional]
**user_id** | Option<**i32**> | The unique ID of the user.  | [optional]
**bot_owner_id** | Option<**i32**> | If the user is a bot (i.e. `is_bot` is `True`), `bot_owner` is the user ID of the bot's owner (usually, whoever created the bot).  Will be null for legacy bots that do not have an owner.  **Changes**: New in Zulip 3.0 (feature level 1).  In previous versions, there was a `bot_owner` field containing the email address of the bot's owner.  | [optional]
**is_active** | Option<**bool**> | A boolean specifying whether the user account has been deactivated.  | [optional]
**is_guest** | Option<**bool**> | A boolean specifying whether the user is a guest user.  | [optional]
**timezone** | Option<**String**> | The time zone of the user.  | [optional]
**date_joined** | Option<**String**> | The time the user account was created.  | [optional]
**delivery_email** | Option<**String**> | The user's real email address.  This field is present only if [email address visibility](/help/restrict-visibility-of-email-addresses) is limited and you are an administrator with access to real email addresses under the configured policy.  | [optional]
**profile_data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | A dictionary containing custom profile field data for the user. Each entry maps the integer ID of a custom profile field in the organization to a dictionary containing the user's data for that field.  Generally the data includes just a single `value` key; for those custom profile fields supporting Markdown, a `rendered_value` key will also be present.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


