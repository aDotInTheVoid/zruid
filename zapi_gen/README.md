# Rust API client for openapi

Powerful open source group chat


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen
For more information, please visit [https://zulip.com](https://zulip.com)

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *https://example.zulipchat.com/api/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AuthenticationApi* | [**dev_fetch_api_key**](docs/AuthenticationApi.md#dev_fetch_api_key) | **post** /dev_fetch_api_key | 
*AuthenticationApi* | [**fetch_api_key**](docs/AuthenticationApi.md#fetch_api_key) | **post** /fetch_api_key | 
*MessagesApi* | [**add_reaction**](docs/MessagesApi.md#add_reaction) | **post** /messages/{message_id}/reactions | 
*MessagesApi* | [**check_messages_match_narrow**](docs/MessagesApi.md#check_messages_match_narrow) | **get** /messages/matches_narrow | 
*MessagesApi* | [**delete_message**](docs/MessagesApi.md#delete_message) | **delete** /messages/{message_id} | 
*MessagesApi* | [**get_file_temporary_url**](docs/MessagesApi.md#get_file_temporary_url) | **get** /user_uploads/{realm_id_str}/{filename} | 
*MessagesApi* | [**get_message_history**](docs/MessagesApi.md#get_message_history) | **get** /messages/{message_id}/history | 
*MessagesApi* | [**get_messages**](docs/MessagesApi.md#get_messages) | **get** /messages | 
*MessagesApi* | [**get_raw_message**](docs/MessagesApi.md#get_raw_message) | **get** /messages/{message_id} | 
*MessagesApi* | [**mark_all_as_read**](docs/MessagesApi.md#mark_all_as_read) | **post** /mark_all_as_read | 
*MessagesApi* | [**mark_stream_as_read**](docs/MessagesApi.md#mark_stream_as_read) | **post** /mark_stream_as_read | 
*MessagesApi* | [**mark_topic_as_read**](docs/MessagesApi.md#mark_topic_as_read) | **post** /mark_topic_as_read | 
*MessagesApi* | [**remove_reaction**](docs/MessagesApi.md#remove_reaction) | **delete** /messages/{message_id}/reactions | 
*MessagesApi* | [**render_message**](docs/MessagesApi.md#render_message) | **post** /messages/render | 
*MessagesApi* | [**send_message**](docs/MessagesApi.md#send_message) | **post** /messages | 
*MessagesApi* | [**update_message**](docs/MessagesApi.md#update_message) | **patch** /messages/{message_id} | 
*MessagesApi* | [**update_message_flags**](docs/MessagesApi.md#update_message_flags) | **post** /messages/flags | 
*MessagesApi* | [**upload_file**](docs/MessagesApi.md#upload_file) | **post** /user_uploads | 
*RealTimeEventsApi* | [**delete_queue**](docs/RealTimeEventsApi.md#delete_queue) | **delete** /events | 
*RealTimeEventsApi* | [**get_events**](docs/RealTimeEventsApi.md#get_events) | **get** /events | 
*RealTimeEventsApi* | [**real_time_post**](docs/RealTimeEventsApi.md#real_time_post) | **post** /real-time | 
*RealTimeEventsApi* | [**register_queue**](docs/RealTimeEventsApi.md#register_queue) | **post** /register | 
*RealTimeEventsApi* | [**rest_error_handling**](docs/RealTimeEventsApi.md#rest_error_handling) | **post** /rest-error-handling | 
*ServerAndOrganizationsApi* | [**add_linkifier**](docs/ServerAndOrganizationsApi.md#add_linkifier) | **post** /realm/filters | 
*ServerAndOrganizationsApi* | [**create_custom_profile_field**](docs/ServerAndOrganizationsApi.md#create_custom_profile_field) | **post** /realm/profile_fields | 
*ServerAndOrganizationsApi* | [**get_custom_emoji**](docs/ServerAndOrganizationsApi.md#get_custom_emoji) | **get** /realm/emoji | 
*ServerAndOrganizationsApi* | [**get_custom_profile_fields**](docs/ServerAndOrganizationsApi.md#get_custom_profile_fields) | **get** /realm/profile_fields | 
*ServerAndOrganizationsApi* | [**get_linkifiers**](docs/ServerAndOrganizationsApi.md#get_linkifiers) | **get** /realm/filters | 
*ServerAndOrganizationsApi* | [**get_server_settings**](docs/ServerAndOrganizationsApi.md#get_server_settings) | **get** /server_settings | 
*ServerAndOrganizationsApi* | [**remove_linkifier**](docs/ServerAndOrganizationsApi.md#remove_linkifier) | **delete** /realm/filters/{filter_id} | 
*ServerAndOrganizationsApi* | [**reorder_custom_profile_fields**](docs/ServerAndOrganizationsApi.md#reorder_custom_profile_fields) | **patch** /realm/profile_fields | 
*ServerAndOrganizationsApi* | [**upload_custom_emoji**](docs/ServerAndOrganizationsApi.md#upload_custom_emoji) | **post** /realm/emoji/{emoji_name} | 
*StreamsApi* | [**create_big_blue_button_video_call**](docs/StreamsApi.md#create_big_blue_button_video_call) | **get** /calls/bigbluebutton/create | 
*StreamsApi* | [**delete_stream**](docs/StreamsApi.md#delete_stream) | **delete** /streams/{stream_id} | 
*StreamsApi* | [**get_stream_id**](docs/StreamsApi.md#get_stream_id) | **get** /get_stream_id | 
*StreamsApi* | [**get_stream_topics**](docs/StreamsApi.md#get_stream_topics) | **get** /users/me/{stream_id}/topics | 
*StreamsApi* | [**get_streams**](docs/StreamsApi.md#get_streams) | **get** /streams | 
*StreamsApi* | [**get_subscription_status**](docs/StreamsApi.md#get_subscription_status) | **get** /users/{user_id}/subscriptions/{stream_id} | 
*StreamsApi* | [**get_subscriptions**](docs/StreamsApi.md#get_subscriptions) | **get** /users/me/subscriptions | 
*StreamsApi* | [**mute_topic**](docs/StreamsApi.md#mute_topic) | **patch** /users/me/subscriptions/muted_topics | 
*StreamsApi* | [**subscribe**](docs/StreamsApi.md#subscribe) | **post** /users/me/subscriptions | 
*StreamsApi* | [**unsubscribe**](docs/StreamsApi.md#unsubscribe) | **delete** /users/me/subscriptions | 
*StreamsApi* | [**update_stream**](docs/StreamsApi.md#update_stream) | **patch** /streams/{stream_id} | 
*StreamsApi* | [**update_subscription_settings**](docs/StreamsApi.md#update_subscription_settings) | **post** /users/me/subscriptions/properties | 
*StreamsApi* | [**update_subscriptions**](docs/StreamsApi.md#update_subscriptions) | **patch** /users/me/subscriptions | 
*UsersApi* | [**create_user**](docs/UsersApi.md#create_user) | **post** /users | 
*UsersApi* | [**create_user_group**](docs/UsersApi.md#create_user_group) | **post** /user_groups/create | 
*UsersApi* | [**deactivate_my_account**](docs/UsersApi.md#deactivate_my_account) | **delete** /users/me | 
*UsersApi* | [**deactivate_user**](docs/UsersApi.md#deactivate_user) | **delete** /users/{user_id} | 
*UsersApi* | [**get_attachments**](docs/UsersApi.md#get_attachments) | **get** /attachments | 
*UsersApi* | [**get_own_user**](docs/UsersApi.md#get_own_user) | **get** /users/me | 
*UsersApi* | [**get_user**](docs/UsersApi.md#get_user) | **get** /users/{user_id} | 
*UsersApi* | [**get_user_by_email**](docs/UsersApi.md#get_user_by_email) | **get** /users/{email} | 
*UsersApi* | [**get_user_groups**](docs/UsersApi.md#get_user_groups) | **get** /user_groups | 
*UsersApi* | [**get_user_presence**](docs/UsersApi.md#get_user_presence) | **get** /users/{email}/presence | 
*UsersApi* | [**get_users**](docs/UsersApi.md#get_users) | **get** /users | 
*UsersApi* | [**reactivate_user**](docs/UsersApi.md#reactivate_user) | **post** /users/{user_id}/reactivate | 
*UsersApi* | [**remove_user_group**](docs/UsersApi.md#remove_user_group) | **delete** /user_groups/{user_group_id} | 
*UsersApi* | [**set_typing_status**](docs/UsersApi.md#set_typing_status) | **post** /typing | 
*UsersApi* | [**update_notification_settings**](docs/UsersApi.md#update_notification_settings) | **patch** /settings/notifications | 
*UsersApi* | [**update_user**](docs/UsersApi.md#update_user) | **patch** /users/{user_id} | 
*UsersApi* | [**update_user_group**](docs/UsersApi.md#update_user_group) | **patch** /user_groups/{user_group_id} | 
*UsersApi* | [**update_user_group_members**](docs/UsersApi.md#update_user_group_members) | **post** /user_groups/{user_group_id}/members | 
*WebhooksApi* | [**zulip_outgoing_webhooks**](docs/WebhooksApi.md#zulip_outgoing_webhooks) | **post** /zulip-outgoing-webhook | 


## Documentation For Models

 - [AddSubscriptionsResponse](docs/AddSubscriptionsResponse.md)
 - [AddSubscriptionsResponseAllOf](docs/AddSubscriptionsResponseAllOf.md)
 - [ApiKeyResponse](docs/ApiKeyResponse.md)
 - [ApiKeyResponseAllOf](docs/ApiKeyResponseAllOf.md)
 - [Attachments](docs/Attachments.md)
 - [AttachmentsMessages](docs/AttachmentsMessages.md)
 - [BadEventQueueIdError](docs/BadEventQueueIdError.md)
 - [BadEventQueueIdErrorAllOf](docs/BadEventQueueIdErrorAllOf.md)
 - [BasicBot](docs/BasicBot.md)
 - [BasicBotAllOf](docs/BasicBotAllOf.md)
 - [BasicBotBase](docs/BasicBotBase.md)
 - [BasicStream](docs/BasicStream.md)
 - [BasicStreamAllOf](docs/BasicStreamAllOf.md)
 - [BasicStreamBase](docs/BasicStreamBase.md)
 - [Bot](docs/Bot.md)
 - [BotAllOf](docs/BotAllOf.md)
 - [CodedError](docs/CodedError.md)
 - [CodedErrorAllOf](docs/CodedErrorAllOf.md)
 - [CodedErrorBase](docs/CodedErrorBase.md)
 - [CodedErrorBaseAllOf](docs/CodedErrorBaseAllOf.md)
 - [CustomProfileField](docs/CustomProfileField.md)
 - [DefaultStreamGroup](docs/DefaultStreamGroup.md)
 - [EmojiReaction](docs/EmojiReaction.md)
 - [EmojiReactionAllOf](docs/EmojiReactionAllOf.md)
 - [EmojiReactionBase](docs/EmojiReactionBase.md)
 - [EmojiReactionBaseUser](docs/EmojiReactionBaseUser.md)
 - [GetMessages](docs/GetMessages.md)
 - [GetMessagesAllOf](docs/GetMessagesAllOf.md)
 - [Hotspot](docs/Hotspot.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [InvalidApiKeyError](docs/InvalidApiKeyError.md)
 - [InvalidMessageError](docs/InvalidMessageError.md)
 - [InvalidMessageErrorAllOf](docs/InvalidMessageErrorAllOf.md)
 - [JsonError](docs/JsonError.md)
 - [JsonErrorBase](docs/JsonErrorBase.md)
 - [JsonErrorBaseAllOf](docs/JsonErrorBaseAllOf.md)
 - [JsonResponseBase](docs/JsonResponseBase.md)
 - [JsonSuccess](docs/JsonSuccess.md)
 - [JsonSuccessAllOf](docs/JsonSuccessAllOf.md)
 - [JsonSuccessBase](docs/JsonSuccessBase.md)
 - [JsonSuccessBaseAllOf](docs/JsonSuccessBaseAllOf.md)
 - [Messages](docs/Messages.md)
 - [MessagesAllOf](docs/MessagesAllOf.md)
 - [MessagesBase](docs/MessagesBase.md)
 - [MissingArgumentError](docs/MissingArgumentError.md)
 - [MissingArgumentErrorAllOf](docs/MissingArgumentErrorAllOf.md)
 - [NonExistingStreamError](docs/NonExistingStreamError.md)
 - [NonExistingStreamErrorAllOf](docs/NonExistingStreamErrorAllOf.md)
 - [Presence](docs/Presence.md)
 - [RealmDomain](docs/RealmDomain.md)
 - [RealmEmoji](docs/RealmEmoji.md)
 - [RealmExport](docs/RealmExport.md)
 - [Subscriptions](docs/Subscriptions.md)
 - [User](docs/User.md)
 - [UserAllOf](docs/UserAllOf.md)
 - [UserBase](docs/UserBase.md)
 - [UserGroup](docs/UserGroup.md)
 - [UserNotAuthorizedError](docs/UserNotAuthorizedError.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



