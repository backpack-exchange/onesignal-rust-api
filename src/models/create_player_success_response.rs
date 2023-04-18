/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.2.1
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePlayerSuccessResponse {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreatePlayerSuccessResponse {
    pub fn new() -> CreatePlayerSuccessResponse {
        CreatePlayerSuccessResponse {
            success: None,
            id: None,
        }
    }
}


