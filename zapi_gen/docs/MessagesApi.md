# \MessagesApi

All URIs are relative to *https://example.zulipchat.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_reaction**](MessagesApi.md#add_reaction) | **post** /messages/{message_id}/reactions | 
[**check_messages_match_narrow**](MessagesApi.md#check_messages_match_narrow) | **get** /messages/matches_narrow | 
[**delete_message**](MessagesApi.md#delete_message) | **delete** /messages/{message_id} | 
[**get_file_temporary_url**](MessagesApi.md#get_file_temporary_url) | **get** /user_uploads/{realm_id_str}/{filename} | 
[**get_message_history**](MessagesApi.md#get_message_history) | **get** /messages/{message_id}/history | 
[**get_messages**](MessagesApi.md#get_messages) | **get** /messages | 
[**get_raw_message**](MessagesApi.md#get_raw_message) | **get** /messages/{message_id} | 
[**mark_all_as_read**](MessagesApi.md#mark_all_as_read) | **post** /mark_all_as_read | 
[**mark_stream_as_read**](MessagesApi.md#mark_stream_as_read) | **post** /mark_stream_as_read | 
[**mark_topic_as_read**](MessagesApi.md#mark_topic_as_read) | **post** /mark_topic_as_read | 
[**remove_reaction**](MessagesApi.md#remove_reaction) | **delete** /messages/{message_id}/reactions | 
[**render_message**](MessagesApi.md#render_message) | **post** /messages/render | 
[**send_message**](MessagesApi.md#send_message) | **post** /messages | 
[**update_message**](MessagesApi.md#update_message) | **patch** /messages/{message_id} | 
[**update_message_flags**](MessagesApi.md#update_message_flags) | **post** /messages/flags | 
[**upload_file**](MessagesApi.md#upload_file) | **post** /user_uploads | 



## add_reaction

> crate::models::JsonSuccess add_reaction(message_id, emoji_name, emoji_code, reaction_type)


Add an [emoji reaction](/help/emoji-reactions) to a message.  `POST {{ api_url }}/v1/messages/{message_id}/reactions` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **i32** | The target message's ID.  | [required] |
**emoji_name** | **String** | The target emoji's human-readable name.  To find an emoji's name, hover over a message to reveal three icons on the right, then click the smiley face icon. Images of available reaction emojis appear. Hover over the emoji you want, and note that emoji's text name.  | [required] |
**emoji_code** | Option<**String**> | A unique identifier, defining the specific emoji codepoint requested, within the namespace of the `reaction_type`.  For most API clients, you won't need this, but it's important for Zulip apps to handle rare corner cases when adding/removing votes on an emoji reaction added previously by another user.  If the existing reaction was added when the Zulip server was using a previous version of the emoji data mapping between Unicode codepoints and human-readable names, sending the `emoji_code` in the data for the original reaction allows the Zulip server to correctly interpret your upvote as an upvote rather than a reaction with a \"diffenent\" emoji.  |  |
**reaction_type** | Option<**String**> | If an app is adding/removing a vote on an existing reaction, it should pass this parameter using the value the server provided for the existing reaction for specificity.  Supported values:  * `unicode_emoji`: Unicode emoji (`emoji_code` will be its Unicode codepoint). * `realm_emoji`: Custom emoji. (`emoji_code` will be its ID). * `zulip_extra_emoji`: Special emoji included with Zulip.  Exists to    namespace the `zulip` emoji.  **Changes**: In Zulip 3.0 (feature level 2), this become optional for [custom emoji](/help/add-custom-emoji); previously, this endpoint assumed `unicode_emoji` if this parameter was not specified.  |  |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_messages_match_narrow

> crate::models::JsonSuccessBase check_messages_match_narrow(msg_ids, narrow)


Check whether a set of messages match a [narrow](/api/construct-narrow).  `GET {{ api_url }}/v1/messages/matches_narrow`  For many common narrows (E.g. a topic), clients can write an efficient client-side check to determine whether a newly arrived message belongs in the view.  This endpoint is designed to allow clients to handle more complex narrows for which the client does not (or in the case of full-text search, cannot) implement this check.  The format of the `match_subject` and `match_content` objects is designed to match those of `GET /messages`, so that a client can splice these fields into a `message` object received from `GET /events` and end up with an extended message object identical to how a `GET /messages` for the current narrow would have returned the message. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**msg_ids** | [**Vec<i32>**](i32.md) | List of IDs for the messages to check. | [required] |
**narrow** | [**Vec<serde_json::Value>**](serde_json::Value.md) | A structure defining the narrow to check against. See how to [construct a narrow](/api/construct-narrow). | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_message

> crate::models::JsonSuccess delete_message(message_id)


Permanently delete a message.  `DELETE {{ api_url }}/v1/messages/{msg_id}`  This API corresponds to the [delete a message completely][delete-completely] feature documented in the Zulip Help Center. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **i32** | The target message's ID.  | [required] |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_temporary_url

> crate::models::JsonSuccessBase get_file_temporary_url(realm_id_str, filename)


Get a temporary URL for access to the file that doesn't require authentication. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**realm_id_str** | **i32** | The realm id.  | [required] |
**filename** | **String** | Path to the URL.  | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message_history

> crate::models::JsonSuccessBase get_message_history(message_id)


Fetch the message edit history of a previously edited message.  `GET {{ api_url }}/v1/messages/{message_id}/history`  Note that edit history may be disabled in some organizations; see the [Zulip Help Center documentation on editing messages][edit-settings].  [edit-settings]: /help/view-a-messages-edit-history 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **i32** | The target message's ID.  | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_messages

> crate::models::JsonSuccessBase get_messages(num_before, num_after, anchor, narrow, client_gravatar, apply_markdown, use_first_unread_anchor)


Fetch message history from a Zulip server.  `GET {{ api_url }}/v1/messages`  This `GET /api/v1/messages` endpoint is the primary way to fetch message history from a Zulip server.  It is useful both for Zulip clients (e.g. the web, desktop, mobile, and terminal clients) as well as bots, API clients, backup scripts, etc.  By specifying a [narrow filter](/api/construct-narrow), you can use this endpoint to fetch the messages matching any search query that is supported by Zulip's powerful full-text search backend.  When a narrow is not specified, it can be used to fetch a user's message history. (We recommend paginating to 1000 messages at a time.)  In either case, you specify an `anchor` message (or ask the server to calculate the first unread message for you and use that as the anchor), as well as a number of messages before and after the anchor message.  The server returns those messages, sorted by message ID, as well as some metadata that makes it easy for a client to determine whether there are more messages matching the query that were not returned due to the `num_before` and `num_after` limits.  We recommend using `num_before <= 1000` and `num_after <= 1000` to avoid generating very large HTTP responses. A maximum of 5000 messages can be obtained per request; attempting to exceed this will result in an error. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**num_before** | **i32** | The number of messages with IDs less than the anchor to retrieve.  | [required] |
**num_after** | **i32** | The number of messages with IDs greater than the anchor to retrieve.  | [required] |
**anchor** | Option<[**crate::models::OneOfstringinteger**](.md)> | Integer message ID to anchor fetching of new messages. Supports special string values for when the client wants the server to compute the anchor to use:  * `newest`: The most recent message. * `oldest`: The oldest message. * `first_unread`: The oldest unread message matching the   query, if any; otherwise, the most recent message.  The special values of `'newest'` and `'oldest'` are also supported for anchoring the query at the most recent or oldest messages.  **Changes**: String values are new in Zulip 3.0 (feature level 1).  The   `first_unread` functionality was supported in Zulip 2.1.x   and older by not sending anchor and using use_first_unread_anchor.    In Zulip 2.1.x and older, `oldest` can be emulated with   `anchor=0`, and `newest` with `anchor=10000000000000000`   (that specific large value works around a bug in Zulip   2.1.x and older in the `found_newest` return value).  |  |
**narrow** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | The narrow where you want to fetch the messages from. See how to [construct a narrow](/api/construct-narrow).  |  |[default to []]
**client_gravatar** | Option<**bool**> | Whether the client supports computing gravatars URLs.  If enabled, `avatar_url` will be included in the response only if there is a Zulip avatar, and will be `null` for users who are using gravatar as their avatar.  This option significantly reduces the compressed size of user data, since gravatar URLs are long, random strings and thus do not compress well. The `client_gravatar` field is set to `true` if clients can compute their own gravatars.  |  |[default to false]
**apply_markdown** | Option<**bool**> | If `true`, message content is returned in the rendered HTML format. If `false`, message content is returned in the raw Markdown-format text that user entered.  |  |[default to true]
**use_first_unread_anchor** | Option<**bool**> | Legacy way to specify `anchor=\"first_unread\"` in Zulip 2.1.x and older.  Whether to use the (computed by the server) first unread message matching the narrow as the `anchor`.  Mutually exclusive with `anchor`.  **Changes**: Deprecated in Zulip 3.0, replaced by `anchor=\"first_unread\"` instead.  |  |[default to false]

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_message

> crate::models::JsonSuccessBase get_raw_message(message_id)


Get the raw content of a message.  `GET {{ api_url }}/v1/messages/{msg_id}`  This is a rarely-used endpoint relevant for clients that primarily work with HTML-rendered messages but might need to occasionally fetch the message's raw Markdown (e.g. for pre-filling a message-editing UI). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **i32** | The target message's ID.  | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_all_as_read

> crate::models::JsonSuccess mark_all_as_read()


Marks all of the current user's unread messages as read.  `POST {{ api_url }}/v1/mark_all_as_read` 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_stream_as_read

> crate::models::JsonSuccess mark_stream_as_read(stream_id)


Mark all the unread messages in a stream as read. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **i32** | The ID of the stream to access.  | [required] |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_topic_as_read

> crate::models::JsonSuccess mark_topic_as_read(stream_id, topic_name)


Mark all the unread messages in a topic as read. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **i32** | The ID of the stream to access.  | [required] |
**topic_name** | **String** | The name of the topic whose messages should be marked as read.  | [required] |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_reaction

> crate::models::JsonSuccess remove_reaction(message_id, emoji_name, emoji_code, reaction_type)


Remove an [emoji reaction](/help/emoji-reactions) from a message.  `DELETE {{ api_url }}/v1/messages/{message_id}/reactions` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **i32** | The target message's ID.  | [required] |
**emoji_name** | Option<**String**> | The target emoji's human-readable name.  To find an emoji's name, hover over a message to reveal three icons on the right, then click the smiley face icon. Images of available reaction emojis appear. Hover over the emoji you want, and note that emoji's text name.  |  |
**emoji_code** | Option<**String**> | A unique identifier, defining the specific emoji codepoint requested, within the namespace of the `reaction_type`.  For most API clients, you won't need this, but it's important for Zulip apps to handle rare corner cases when adding/removing votes on an emoji reaction added previously by another user.  If the existing reaction was added when the Zulip server was using a previous version of the emoji data mapping between Unicode codepoints and human-readable names, sending the `emoji_code` in the data for the original reaction allows the Zulip server to correctly interpret your upvote as an upvote rather than a reaction with a \"diffenent\" emoji.  |  |
**reaction_type** | Option<**String**> | If an app is adding/removing a vote on an existing reaction, it should pass this parameter using the value the server provided for the existing reaction for specificity.  Supported values:  * `unicode_emoji`: Unicode emoji (`emoji_code` will be its Unicode codepoint). * `realm_emoji`: Custom emoji. (`emoji_code` will be its ID). * `zulip_extra_emoji`: Special emoji included with Zulip.  Exists to    namespace the `zulip` emoji.  **Changes**: In Zulip 3.0 (feature level 2), this become optional for [custom emoji](/help/add-custom-emoji); previously, this endpoint assumed `unicode_emoji` if this parameter was not specified.  |  |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_message

> crate::models::JsonSuccessBase render_message(content)


Render a message to HTML.  `POST {{ api_url }}/v1/messages/render` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content** | **String** | The content of the message. Maximum message size of 10000 bytes.  | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_message

> crate::models::JsonSuccessBase send_message(_type, to, content, topic, queue_id, local_id)


Send a stream or a private message.  `POST {{ api_url }}/v1/messages` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** | The type of message to be sent. `private` for a private message and `stream` for a stream message.  | [required] |
**to** | [**Vec<i32>**](i32.md) | For stream messages, either the name or integer ID of the stream. For private messages, either a list containing integer user IDs or a list containing string email addresses.  **Changes**: Support for using user/stream IDs was added in Zulip 2.0.0.  | [required] |
**content** | **String** | The content of the message. Maximum message size of 10000 bytes.  | [required] |
**topic** | Option<**String**> | The topic of the message. Only required for stream messages (`type=\"stream\"`), ignored otherwise.  Maximum length of 60 characters.  **Changes**: New in Zulip 2.0.  Previous Zulip releases encoded this as `subject`, which is currently a deprecated alias.  |  |
**queue_id** | Option<**String**> | For clients supporting [local echo](https://zulip.readthedocs.io/en/latest/subsystems/sending-messages.html#local-echo), the [event queue](/api/register-queue) ID for the client.  If passed, `local_id` is required.  If the message is successfully sent, the server will include `local_id` in the `message` event that the client with this `queue_id` will receive notifying it of the new message via [`GET /events`](/api/get-events).  This lets the client know unambiguously that it should replace the locally echoed message, rather than adding this new message (which would be correct if the user had sent the new message from another device).  |  |
**local_id** | Option<**String**> | For clients supporting local echo, a unique string-format identifier chosen freely by the client; the server will pass it back to the client without inspecting it, as described in the `queue_id` description.  |  |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_message

> crate::models::JsonSuccess update_message(message_id, topic, propagate_mode, send_notification_to_old_thread, send_notification_to_new_thread, content, stream_id)


Edit/update the content or topic of a message.  `PATCH {{ api_url }}/v1/messages/{msg_id}`  `{msg_id}` in the above URL should be replaced with the ID of the message you wish you update. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **i32** | The target message's ID.  | [required] |
**topic** | Option<**String**> | The topic of the message. Only required for stream messages (`type=\"stream\"`), ignored otherwise.  Maximum length of 60 characters.  **Changes**: New in Zulip 2.0.  Previous Zulip releases encoded this as `subject`, which is currently a deprecated alias.  |  |
**propagate_mode** | Option<**String**> | Which message(s) should be edited: just the one indicated in `message_id`, messages in the same topic that had been sent after this one, or all of them.  |  |[default to change_one]
**send_notification_to_old_thread** | Option<**bool**> | Whether to send breadcrumb message to the old thread to notify users where the messages were moved to.  **Changes**: New in Zulip 3.0 (feature level 9).  |  |[default to true]
**send_notification_to_new_thread** | Option<**bool**> | Whether to send a notification message to the new thread to notify users where the messages came from.  **Changes**: New in Zulip 3.0 (feature level 9).  |  |[default to true]
**content** | Option<**String**> | The content of the message. Maximum message size of 10000 bytes.  |  |
**stream_id** | Option<**i32**> | The ID of the stream to access.  |  |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_message_flags

> crate::models::JsonSuccessBase update_message_flags(messages, op, flag)


Add or remove personal message flags like `read` and `starred` on a collection of message IDs.  `POST {{ api_url }}/v1/messages/flags`  For updating the `read` flag on common collections of messages, see also the [special endpoints for marking message as read in bulk](/api/mark-all-as-read). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messages** | [**Vec<i32>**](i32.md) | An array containing the IDs of the target messages.  | [required] |
**op** | **String** | Whether to `add` the flag or `remove` it.  | [required] |
**flag** | **String** | The flag that should be added/removed.  | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file

> crate::models::JsonSuccessBase upload_file(filename)


Upload a single file and get the corresponding URI.  `POST {{ api_url }}/v1/user_uploads`  Initially, only you will be able to access the link.  To share the uploaded file, you'll need to [send a message][send-message] containing the resulting link.  Users who can already access the link can reshare it with other users by sending additional Zulip messages containing the link.  [uploaded-files]: /help/manage-your-uploaded-files [send-message]: /api/send-message 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filename** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

