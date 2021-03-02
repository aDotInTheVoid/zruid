# \StreamsApi

All URIs are relative to *https://example.zulipchat.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_big_blue_button_video_call**](StreamsApi.md#create_big_blue_button_video_call) | **get** /calls/bigbluebutton/create | 
[**delete_stream**](StreamsApi.md#delete_stream) | **delete** /streams/{stream_id} | 
[**get_stream_id**](StreamsApi.md#get_stream_id) | **get** /get_stream_id | 
[**get_stream_topics**](StreamsApi.md#get_stream_topics) | **get** /users/me/{stream_id}/topics | 
[**get_streams**](StreamsApi.md#get_streams) | **get** /streams | 
[**get_subscription_status**](StreamsApi.md#get_subscription_status) | **get** /users/{user_id}/subscriptions/{stream_id} | 
[**get_subscriptions**](StreamsApi.md#get_subscriptions) | **get** /users/me/subscriptions | 
[**mute_topic**](StreamsApi.md#mute_topic) | **patch** /users/me/subscriptions/muted_topics | 
[**subscribe**](StreamsApi.md#subscribe) | **post** /users/me/subscriptions | 
[**unsubscribe**](StreamsApi.md#unsubscribe) | **delete** /users/me/subscriptions | 
[**update_stream**](StreamsApi.md#update_stream) | **patch** /streams/{stream_id} | 
[**update_subscription_settings**](StreamsApi.md#update_subscription_settings) | **post** /users/me/subscriptions/properties | 
[**update_subscriptions**](StreamsApi.md#update_subscriptions) | **patch** /users/me/subscriptions | 



## create_big_blue_button_video_call

> crate::models::JsonSuccessBase create_big_blue_button_video_call()


Create a video call URL for a Big Blue Button video call. Requires Big Blue Button to be configured on the Zulip server. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_stream

> crate::models::JsonSuccess delete_stream(stream_id)


[Delete the stream](/help/delete-a-stream) with the ID `stream_id`.  `DELETE {{ api_url }}/v1/streams/{stream_id}` 

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


## get_stream_id

> crate::models::JsonSuccessBase get_stream_id(stream)


Get the unique ID of a given stream.  `GET {{ api_url }}/v1/get_stream_id` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream** | **String** | The name of the stream to access.  | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stream_topics

> crate::models::JsonSuccessBase get_stream_topics(stream_id)


Get all the topics in a specific stream  `GET {{ api_url }}/v1/users/me/{stream_id}/topics` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **i32** | The ID of the stream to access.  | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_streams

> crate::models::JsonSuccessBase get_streams(include_public, include_web_public, include_subscribed, include_all_active, include_default, include_owner_subscribed)


Get all streams that the user has access to.  `GET {{ api_url }}/v1/streams` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_public** | Option<**bool**> | Include all public streams.  |  |[default to true]
**include_web_public** | Option<**bool**> | Include all web public streams.  |  |[default to false]
**include_subscribed** | Option<**bool**> | Include all streams that the user is subscribed to.  |  |[default to true]
**include_all_active** | Option<**bool**> | Include all active streams. The user must have administrative privileges to use this parameter.  |  |[default to false]
**include_default** | Option<**bool**> | Include all default streams for the user's realm.  |  |[default to false]
**include_owner_subscribed** | Option<**bool**> | If the user is a bot, include all streams that the bot's owner is subscribed to.  |  |[default to false]

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_status

> crate::models::JsonSuccessBase get_subscription_status(user_id, stream_id)


Check whether a user is subscribed to a stream.  `GET {{ api_url }}/v1/users/{user_id}/subscriptions/{stream_id}`  **Changes**: New in Zulip 3.0 (feature level 11). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | The target user's ID.  | [required] |
**stream_id** | **i32** | The ID of the stream to access.  | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscriptions

> crate::models::JsonSuccessBase get_subscriptions(include_subscribers)


Get all streams that the user is subscribed to.  `GET {{ api_url }}/v1/users/me/subscriptions` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_subscribers** | Option<**bool**> | Whether each returned stream object should include a `subscribers` field containing a list of the user IDs of its subscribers.  (This may be significantly slower in organizations with thousands of users subscribed to many streams.)  **Changes**: New in Zulip 2.1.0.  |  |[default to false]

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mute_topic

> crate::models::JsonSuccess mute_topic(topic, op, stream, stream_id)


This endpoint mutes/unmutes a topic within a stream that the current user is subscribed to.  Muted topics are displayed faded in the Zulip UI, and are not included in the user's unread count totals.  `PATCH {{ api_url }}/v1/users/me/subscriptions/muted_topics` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic** | **String** | The topic to (un)mute. Note that the request will succeed regardless of whether any messages have been sent to the specified topic.  | [required] |
**op** | **String** | Whether to mute (`add`) or unmute (`remove`) the provided topic.  | [required] |
**stream** | Option<**String**> | The name of the stream to access.  |  |
**stream_id** | Option<**i32**> | The ID of the stream to access.  |  |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe

> crate::models::OneOfobjectobject subscribe(subscriptions, principals, authorization_errors_fatal, announce, invite_only, history_public_to_subscribers, stream_post_policy, message_retention_days)


Subscribe one or more users to one or more streams.  `POST {{ api_url }}/v1/users/me/subscriptions`  If any of the specified streams do not exist, they are automatically created.  The initial [stream settings](/api/update-stream) will be determined by the optional parameters like `invite_only` detailed below. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscriptions** | [**Vec<serde_json::Value>**](serde_json::Value.md) | A list of dictionaries containing the key `name` and value specifying the name of the stream to subscribe. If the stream does not exist a new stream is created. The description of the stream created can be specified by setting the dictionary key `description` with an appropriate value.  | [required] |
**principals** | Option<[**Vec<crate::models::OneOfstringinteger>**](crate::models::OneOfstringinteger.md)> | A list of user ids (preferred) or Zulip display email addresses of the users to be subscribed to or unsubscribed from the streams specified in the `subscriptions` parameter. If not provided, then the requesting user/bot is subscribed.  **Changes**: The integer format is new in Zulip 3.0 (feature level 9).  |  |
**authorization_errors_fatal** | Option<**bool**> | A boolean specifying whether authorization errors (such as when the requesting user is not authorized to access a private stream) should be considered fatal or not. When `True`, an authorization error is reported as such. When set to `False`, the response will be a 200 and any streams where the request encountered an authorization error will be listed in the `unauthorized` key.  |  |[default to true]
**announce** | Option<**bool**> | If one of the streams specified did not exist previously and is thus craeted by this call, this determines whether [notification bot](/help/configure-notification-bot) will send an announcement about the new stream's creation.  |  |[default to false]
**invite_only** | Option<**bool**> | As described above, this endpoint will create a new stream if passed a stream name that doesn't already exist.  This parameters and the ones that follow are used to request an initial configuration of a created stream; they are ignored for streams that already exist.  This parameter determines whether any newly created streams will be private streams.  |  |[default to false]
**history_public_to_subscribers** | Option<**bool**> | Whether the stream's message history should be available to newly subscribed members, or users can only access messages they actually received while subscribed to the stream.  Corresponds to the [shared history](/help/stream-permissions) option in documentation.  |  |
**stream_post_policy** | Option<**i32**> | Policy for which users can post messages to the stream.  * 1 => Any user can post. * 2 => Only administrators can post. * 3 => Only new members can post.  **Changes**: New in Zulip 3.0, replacing the previous `is_announcement_only` boolean.  |  |[default to 1]
**message_retention_days** | Option<[**crate::models::OneOfstringinteger**](.md)> | Number of days that messages sent to this stream will be stored before being automatically deleted by the [message retention policy](/help/message-retention-policy).  Two special string format values are supported:  * \"realm_default\" => Return to the organization-level setting. * \"forever\" => Retain messages forever.  **Changes**: New in Zulip 3.0 (feature level 17).  |  |

### Return type

[**crate::models::OneOfobjectobject**](oneOf<object,object>.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe

> crate::models::JsonSuccessBase unsubscribe(subscriptions, principals)


Unsubscribe yourself or other users from one or more streams.  `DELETE {{ api_url }}/v1/users/me/subscriptions` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscriptions** | [**Vec<String>**](String.md) | A list of stream names to unsubscribe from. This parameter is called `streams` in our Python API.  | [required] |
**principals** | Option<[**Vec<crate::models::OneOfstringinteger>**](crate::models::OneOfstringinteger.md)> | A list of user ids (preferred) or Zulip display email addresses of the users to be subscribed to or unsubscribed from the streams specified in the `subscriptions` parameter. If not provided, then the requesting user/bot is subscribed.  **Changes**: The integer format is new in Zulip 3.0 (feature level 9).  |  |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_stream

> crate::models::JsonSuccess update_stream(stream_id, description, new_name, is_private, is_announcement_only, stream_post_policy, history_public_to_subscribers, message_retention_days)


Configure the stream with the ID `stream_id`.  This endpoint supports an organization administrator editing any property of a stream, including:  * Stream [name](/help/rename-a-stream) and [description](/help/change-the-stream-description) * Stream [permissions](/help/stream-permissions), including [privacy](/help/change-the-privacy-of-a-stream) and [who can send](/help/stream-sending-policy).  `PATCH {{ api_url }}/v1/streams/{stream_id}` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **i32** | The ID of the stream to access.  | [required] |
**description** | Option<**String**> | The new description for the stream.  |  |
**new_name** | Option<**String**> | The new name for the stream.  |  |
**is_private** | Option<**bool**> | Change whether the stream is a private stream.  |  |
**is_announcement_only** | Option<**bool**> | Whether the stream is limited to announcements.  **Changes**: Deprecated in Zulip 3.0 (feature level 1), use   `stream_post_policy` instead.  |  |
**stream_post_policy** | Option<**i32**> | Policy for which users can post messages to the stream.  * 1 => Any user can post. * 2 => Only administrators can post. * 3 => Only new members can post.  **Changes**: New in Zulip 3.0, replacing the previous `is_announcement_only` boolean.  |  |[default to 1]
**history_public_to_subscribers** | Option<**bool**> | Whether the stream's message history should be available to newly subscribed members, or users can only access messages they actually received while subscribed to the stream.  Corresponds to the [shared history](/help/stream-permissions) option in documentation.  |  |
**message_retention_days** | Option<[**crate::models::OneOfstringinteger**](.md)> | Number of days that messages sent to this stream will be stored before being automatically deleted by the [message retention policy](/help/message-retention-policy).  Two special string format values are supported:  * \"realm_default\" => Return to the organization-level setting. * \"forever\" => Retain messages forever.  **Changes**: New in Zulip 3.0 (feature level 17).  |  |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription_settings

> crate::models::JsonSuccessBase update_subscription_settings(subscription_data)


This endpoint is used to update the user's personal settings for the streams they are subscribed to, including muting, color, pinning, and per-stream notification settings.  `POST {{ api_url }}/v1/users/me/subscriptions/properties` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_data** | [**Vec<serde_json::Value>**](serde_json::Value.md) | A list of objects that describe the changes that should be applied in each subscription. Each object represents a subscription, and must have a `stream_id` key that identifies the stream, as well as the `property` being modified and its new `value`.  The possible values for each `property` and `value` pairs are:  * `color` (string): the hex value of the user's display color for the stream. * `is_muted` (boolean): whether the stream is   [muted](/help/mute-a-stream).  Prior to Zulip 2.1, this feature was   represented by the more confusingly named `in_home_view` (with the   opposite value, `in_home_view=!is_muted`); for   backwards-compatibility, modern Zulip still accepts that value. * `pin_to_top` (boolean): whether to pin the stream at the top of the stream list. * `desktop_notifications` (boolean): whether to show desktop notifications     for all messages sent to the stream. * `audible_notifications` (boolean): whether to play a sound   notification for all messages sent to the stream. * `push_notifications` (boolean): whether to trigger a mobile push     notification for all messages sent to the stream. * `email_notifications` (boolean): whether to trigger an email     notification for all messages sent to the stream.  | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscriptions

> crate::models::JsonSuccessBase update_subscriptions(delete, add)


Update which streams you are are subscribed to. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete** | Option<[**Vec<String>**](String.md)> | A list of stream names to unsubscribe from.  |  |
**add** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | A list of objects describing which streams to subscribe to, optionally including per-user subscription parameters (e.g. color) and if the stream is to be created, its description.  |  |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

