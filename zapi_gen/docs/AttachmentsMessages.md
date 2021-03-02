# AttachmentsMessages

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_sent** | Option<**i32**> | Time when the message was sent as a UNIX timestamp multiplied by 1000 (matching the format of getTime() in JavaScript).  **Changes**: Changed in Zulip 2.2 (feature level 22).  This field was previously strangely called `name` and was a floating point number.  | [optional]
**id** | Option<**i32**> | The unique message ID.  Messages should always be displayed sorted by ID.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


