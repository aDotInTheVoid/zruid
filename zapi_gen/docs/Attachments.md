# Attachments

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID for the attachment.  | [optional]
**name** | Option<**String**> | Name of the uploaded file.  | [optional]
**path_id** | Option<**String**> | A representation of the path of the file within the repository of user-uploaded files.  If the `path_id` of a file is `{realm_id}/ab/cdef/temp_file.py`, its URL will be: `{server_url}/user_uploads/{realm_id}/ab/cdef/temp_file.py`.  | [optional]
**size** | Option<**i32**> | Size of the file in bytes.  | [optional]
**create_time** | Option<**i32**> | Time when the attachment was uploaded as a UNIX timestamp multiplied by 1000 (matching the format of getTime() in JavaScript).  **Changes**: Changed in Zulip 2.2 (feature level 22).  This field was previously a floating point number.  | [optional]
**messages** | Option<[**Vec<crate::models::AttachmentsMessages>**](Attachments_messages.md)> | Contains basic details on any Zulip messages that have been sent referencing this [uploaded file](/api/upload-file). This includes messages sent by any user in the Zulip organization who sent a message containing a link to the uploaded file.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


