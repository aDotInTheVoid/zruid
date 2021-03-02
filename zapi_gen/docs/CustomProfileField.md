# CustomProfileField

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the custom profile field.  This will be referenced in custom the profile fields section of user objects.  | [optional]
**_type** | Option<**i32**> | An integer indicating the type of the custom profile field, which determines how it is configured and displayed to users.  See the [Add custom profile fields](/help/add-custom-profile-fields) article for details on what each type means.  * **1**: Short text * **2**: Long text * **3**: List of options * **4**: Date picker * **5**: Link * **6**: Person picker * **7**: External account  | [optional]
**order** | Option<**i32**> | Custom profile fields are displayed in both settings UI and UI showing users' profiles in increasing `order`.  | [optional]
**name** | Option<**String**> | The name of the custom profile field.  | [optional]
**hint** | Option<**String**> | The help text to be displayed for the custom profile field in user-facing settings UI for configuring custom profile fields.  | [optional]
**field_data** | Option<**String**> | Field types 3 (List of options) and 7 (External account) support storing additional configuration for the field type in the `field_data` attribute.  For field type 3 (List of options), this attribute is a JSON dictionary defining the choices and the order they will be displayed in the dropdown UI for individual users to select an option.  The interface for field type 7 is not yet stabilized.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


