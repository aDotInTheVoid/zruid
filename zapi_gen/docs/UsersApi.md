# \UsersApi

All URIs are relative to *https://example.zulipchat.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UsersApi.md#create_user) | **post** /users | 
[**create_user_group**](UsersApi.md#create_user_group) | **post** /user_groups/create | 
[**deactivate_my_account**](UsersApi.md#deactivate_my_account) | **delete** /users/me | 
[**deactivate_user**](UsersApi.md#deactivate_user) | **delete** /users/{user_id} | 
[**get_attachments**](UsersApi.md#get_attachments) | **get** /attachments | 
[**get_own_user**](UsersApi.md#get_own_user) | **get** /users/me | 
[**get_user**](UsersApi.md#get_user) | **get** /users/{user_id} | 
[**get_user_by_email**](UsersApi.md#get_user_by_email) | **get** /users/{email} | 
[**get_user_groups**](UsersApi.md#get_user_groups) | **get** /user_groups | 
[**get_user_presence**](UsersApi.md#get_user_presence) | **get** /users/{email}/presence | 
[**get_users**](UsersApi.md#get_users) | **get** /users | 
[**reactivate_user**](UsersApi.md#reactivate_user) | **post** /users/{user_id}/reactivate | 
[**remove_user_group**](UsersApi.md#remove_user_group) | **delete** /user_groups/{user_group_id} | 
[**set_typing_status**](UsersApi.md#set_typing_status) | **post** /typing | 
[**update_notification_settings**](UsersApi.md#update_notification_settings) | **patch** /settings/notifications | 
[**update_user**](UsersApi.md#update_user) | **patch** /users/{user_id} | 
[**update_user_group**](UsersApi.md#update_user_group) | **patch** /user_groups/{user_group_id} | 
[**update_user_group_members**](UsersApi.md#update_user_group_members) | **post** /user_groups/{user_group_id}/members | 



## create_user

> crate::models::JsonSuccessBase create_user(email, password, full_name)


{!can-create-users-only.md!}  Create a new user account via the API.  `POST {{ api_url }}/v1/users` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | The email address of the new user.  | [required] |
**password** | **String** | The password of the new user.  | [required] |
**full_name** | **String** | The full name of the new user.  | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_group

> crate::models::JsonSuccess create_user_group(name, description, members)


Create a new [user group](/help/user-groups).  `POST {{ api_url }}/v1/user_groups/create` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the user group.  | [required] |
**description** | **String** | The description of the user group.  | [required] |
**members** | [**Vec<i32>**](i32.md) | An array containing the user IDs of the initial members for the new user group.  | [required] |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deactivate_my_account

> crate::models::JsonSuccess deactivate_my_account()


Delete the requesting user from the realm. 

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


## deactivate_user

> crate::models::JsonSuccess deactivate_user(user_id)


{!api-admin-only.md!}  [Deactivates a user](https://zulip.com/help/deactivate-or-reactivate-a-user) given their user ID.  `DELETE {{ api_url }}/v1/users/{user_id}` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | The target user's ID.  | [required] |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attachments

> crate::models::JsonSuccessBase get_attachments()


Fetch metadata on files uploaded by the requesting user.  `GET {{ api_url }}/v1/attachments` 

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


## get_own_user

> crate::models::JsonSuccessBase get_own_user()


Get basic data about the user/bot that requests this endpoint.  `GET {{ api_url }}/v1/users/me` 

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


## get_user

> crate::models::JsonSuccessBase get_user(user_id, client_gravatar, include_custom_profile_fields)


Fetch details for a single user in the organization.  `GET {{ api_url }}/v1/users/{user_id}`  You can also fetch details on [all users in the organization](/api/get-users) or [by email](/api/get-user-by-email).  *This endpoint is new in Zulip Server 3.0 (feature level 1).* 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | The target user's ID.  | [required] |
**client_gravatar** | Option<**bool**> | Whether the client supports computing gravatars URLs.  If enabled, `avatar_url` will be included in the response only if there is a Zulip avatar, and will be `null` for users who are using gravatar as their avatar.  This option significantly reduces the compressed size of user data, since gravatar URLs are long, random strings and thus do not compress well. The `client_gravatar` field is set to `true` if clients can compute their own gravatars.  |  |[default to false]
**include_custom_profile_fields** | Option<**bool**> | Whether the client wants [custom profile field](/help/add-custom-profile-fields) data to be included in the response.  **Changes**: New in Zulip 2.1.0.  Previous versions do no offer these data via the API.  |  |[default to false]

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_email

> crate::models::JsonSuccessBase get_user_by_email(email, client_gravatar, include_custom_profile_fields)


Fetch details for a single user in the organization given a Zulip display email address.  `GET {{ api_url }}/v1/users/{email}`  Note that this endpoint uses Zulip display emails addresses for organizations that have configured limited [email address visibility](/help/restrict-visibility-of-email-addresses).  You can also fetch details on [all users in the organization](/api/get-users) or [by user ID](/api/get-user).  Fetching by user ID is generally recommended when possible, as users can [change their email address](/help/change-your-email-address).  *This endpoint is new in Zulip Server 4.0 (feature level 39).* 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | The email address of the user whose details you want to fetch.  | [required] |
**client_gravatar** | Option<**bool**> | Whether the client supports computing gravatars URLs.  If enabled, `avatar_url` will be included in the response only if there is a Zulip avatar, and will be `null` for users who are using gravatar as their avatar.  This option significantly reduces the compressed size of user data, since gravatar URLs are long, random strings and thus do not compress well. The `client_gravatar` field is set to `true` if clients can compute their own gravatars.  |  |[default to false]
**include_custom_profile_fields** | Option<**bool**> | Whether the client wants [custom profile field](/help/add-custom-profile-fields) data to be included in the response.  **Changes**: New in Zulip 2.1.0.  Previous versions do no offer these data via the API.  |  |[default to false]

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_groups

> crate::models::JsonSuccessBase get_user_groups()


Fetches all of the user groups in the organization.  `GET {{ api_url }}/v1/user_groups` 

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


## get_user_presence

> crate::models::JsonSuccessBase get_user_presence(email)


Get the presence status for a specific user.  This endpoint is most useful for embedding data about a user's presence status in other sites (E.g. an employee directory).  Full Zulip clients like mobile/desktop apps will want to use the main presence endpoint, which returns data for all active users in the organization, instead.  `GET {{ api_url }}/v1/users/{email}/presence`  See [Zulip's developer documentation](https://zulip.readthedocs.io/en/latest/subsystems/presence.html) for details on the data model for presence in Zulip. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | The email address of the user whose presence you want to fetch.  | [required] |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> crate::models::JsonSuccessBase get_users(client_gravatar, include_custom_profile_fields)


Retrieve details on all users in the organization.  Optionally includes values of [custom profile field](/help/add-custom-profile-fields).  `GET {{ api_url }}/v1/users`  You can also [fetch details on a single user](/api/get-user). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_gravatar** | Option<**bool**> | Whether the client supports computing gravatars URLs.  If enabled, `avatar_url` will be included in the response only if there is a Zulip avatar, and will be `null` for users who are using gravatar as their avatar.  This option significantly reduces the compressed size of user data, since gravatar URLs are long, random strings and thus do not compress well. The `client_gravatar` field is set to `true` if clients can compute their own gravatars.  |  |[default to false]
**include_custom_profile_fields** | Option<**bool**> | Whether the client wants [custom profile field](/help/add-custom-profile-fields) data to be included in the response.  **Changes**: New in Zulip 2.1.0.  Previous versions do no offer these data via the API.  |  |[default to false]

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactivate_user

> crate::models::JsonSuccess reactivate_user(user_id)


{!api-admin-only.md!}  [Reactivates a user](https://zulip.com/help/deactivate-or-reactivate-a-user) given their user ID.  `POST {{ api_url }}/v1/users/{user_id}/reactivate` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | The target user's ID.  | [required] |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_group

> crate::models::JsonSuccess remove_user_group(user_group_id)


Delete a [user group](/help/user-groups).  `DELETE {{ api_url }}/v1/user_groups/{user_group_id}` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **i32** | The ID of the target user group.  | [required] |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_typing_status

> crate::models::JsonSuccess set_typing_status(op, to)


Notify other users whether the current user is typing a message.  `POST {{ api_url }}/v1/typing`  Clients implementing Zulip's typing notifications protocol should work as follows:  * Send a request to this endpoint with `op=\"start\"` when a user starts typing   a private message or group private message, and also every   `TYPING_STARTED_WAIT_PERIOD=10` seconds that the user continues to actively type   or otherwise interact with the compose UI (E.g. interacting with the compose box   emoji picker). * Send a request to this endpoint with `op=\"stop\"` when a user pauses using the   compose UI for at least `TYPING_STOPPED_WAIT_PERIOD=5` seconds or cancels   the compose action (if it had previously sent a \"start\" operation for that   compose action). * Start displaying \"Sender is typing\" for a given conversation when the client   receives an `op=\"start\"` event from the [events API](/api/get-events). * Continue displaying \"Sender is typing\" until they receive an `op=\"stop\"` event   from the [events API](/api/get-events) or `TYPING_STARTED_EXPIRY_PERIOD=15`   seconds have passed without a new `op=\"start\"` event for that conversation.  This protocol is designed to allow the server-side typing notifications implementation to be stateless while being resilient; network failures cannot result in a user being incorrectly displayed as perpetually typing.  See [the typing notification docs](https://zulip.readthedocs.io/en/latest/subsystems/typing-indicators.html) for additional design details on Zulip's typing notifications protocol. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**op** | **String** | Whether the user has started (`start`) or stopped (`stop`) to type.  | [required] |
**to** | [**Vec<i32>**](i32.md) | The user_ids of the recipients of the message being typed. Typing notifications are only supported for private messages. Send a JSON-encoded list of user_ids. (Use a list even if there is only one recipient.)  **Changes**: Before Zulip 2.0, this parameter accepted only a JSON-encoded list of email addresses.  Support for the email address-based format was removed in Zulip 3.0 (feature level 11).  | [required] |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notification_settings

> crate::models::JsonSuccessBase update_notification_settings(enable_stream_desktop_notifications, enable_stream_email_notifications, enable_stream_push_notifications, enable_stream_audible_notifications, notification_sound, enable_desktop_notifications, enable_sounds, enable_offline_email_notifications, enable_offline_push_notifications, enable_online_push_notifications, enable_digest_emails, enable_login_emails, message_content_in_email_notifications, pm_content_in_desktop_notifications, wildcard_mentions_notify, desktop_icon_count_display, realm_name_in_notifications, presence_enabled)


This endpoint is used to edit the user's global notification settings. See [this endpoint](/api/update-subscription-settings) for per-stream notification settings.  `PATCH {{ api_url }}/v1/settings/notifications` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enable_stream_desktop_notifications** | Option<**bool**> | Enable visual desktop notifications for stream messages.  |  |
**enable_stream_email_notifications** | Option<**bool**> | Enable email notifications for stream messages.  |  |
**enable_stream_push_notifications** | Option<**bool**> | Enable mobile notifications for stream messages.  |  |
**enable_stream_audible_notifications** | Option<**bool**> | Enable audible desktop notifications for stream messages.  |  |
**notification_sound** | Option<**String**> | Notification sound name.  |  |
**enable_desktop_notifications** | Option<**bool**> | Enable visual desktop notifications for private messages and @-mentions.  |  |
**enable_sounds** | Option<**bool**> | Enable audible desktop notifications for private messages and @-mentions.  |  |
**enable_offline_email_notifications** | Option<**bool**> | Enable email notifications for private messages and @-mentions received when the user is offline.  |  |
**enable_offline_push_notifications** | Option<**bool**> | Enable mobile notification for private messages and @-mentions received when the user is offline.  |  |
**enable_online_push_notifications** | Option<**bool**> | Enable mobile notification for private messages and @-mentions received when the user is online.  |  |
**enable_digest_emails** | Option<**bool**> | Enable digest emails when the user is away.  |  |
**enable_login_emails** | Option<**bool**> | Enable email notifications for new logins to account.  |  |
**message_content_in_email_notifications** | Option<**bool**> | Include the message's content in missed messages email notifications.  |  |
**pm_content_in_desktop_notifications** | Option<**bool**> | Include content of private messages in desktop notifications.  |  |
**wildcard_mentions_notify** | Option<**bool**> | Whether wildcard mentions (E.g. @**all**) should send notifications like a personal mention.  |  |
**desktop_icon_count_display** | Option<**i32**> | Unread count summary (appears in desktop sidebar and browser tab)  * 1 - All unreads * 2 - Private messages and mentions * 3 - None  |  |
**realm_name_in_notifications** | Option<**bool**> | Include organization name in subject of missed message emails.  |  |
**presence_enabled** | Option<**bool**> | Display the presence status to other users when online.  |  |

### Return type

[**crate::models::JsonSuccessBase**](JsonSuccessBase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::JsonSuccess update_user(user_id, full_name, role, profile_data)


{!api-admin-only.md!}  Administrative endpoint to update the details of another user in the organization.  `PATCH {{ api_url }}/v1/users/{user_id}`  Supports everything an administrator can do to edit details of another user's account, including editing full name, [role](/help/roles-and-permissions), and [custom profile fields](/help/add-custom-profile-fields). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | The target user's ID.  | [required] |
**full_name** | Option<**String**> | The user's full name.  |  |
**role** | Option<**i32**> | New [role](/help/roles-and-permissions) for the user.  Roles are encoded as:  * Organization owner: 100 * Organization administrator: 200 * Member: 400 * Guest: 600  Only organization owners can add or remove the owner role.  The owner role cannot be removed from the only organization owner.  **Changes**: New in Zulip 3.0 (feature level 8), replacing the previous pair of `is_admin` and `is_guest` boolean parameters.  |  |
**profile_data** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | A dictionary containing the to be updated custom profile field data for the user.  |  |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_group

> crate::models::JsonSuccess update_user_group(user_group_id, name, description)


Update the name or description of a [user group](/help/user-groups).  `PATCH {{ api_url }}/v1/user_groups/{user_group_id}` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **i32** | The ID of the target user group.  | [required] |
**name** | **String** | The new name of the group.  | [required] |
**description** | **String** | The new description of the group.  | [required] |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_group_members

> crate::models::JsonSuccess update_user_group_members(user_group_id, delete, add)


Update the members of a [user group](/help/user-groups).  `POST {{ api_url }}/v1/user_groups/{user_group_id}/members` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_group_id** | **i32** | The ID of the target user group.  | [required] |
**delete** | Option<[**Vec<i32>**](i32.md)> | The list of user ids to be removed from the user group.  |  |
**add** | Option<[**Vec<i32>**](i32.md)> | The list of user ids to be added to the user group.  |  |

### Return type

[**crate::models::JsonSuccess**](JsonSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

