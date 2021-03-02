# RealmExport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The id of the export.  | [optional]
**acting_user_id** | Option<**i32**> | The id of the user who did the export.  | [optional]
**export_time** | Option<**f32**> | The UNIX timestamp of when the export was made.  | [optional]
**deleted_timestamp** | Option<**f32**> | The timestamp of when the export was deleted. Null if it wasn't.  | [optional]
**failed_timestamp** | Option<**f32**> | The timestamp of when the export failed. Null if it didn't.  | [optional]
**export_url** | Option<**String**> | The URL of the export. `null` if there's no URL.  | [optional]
**pending** | Option<**bool**> | Whether the export is pending or not.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


