/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.2.1
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */

/// BasicNotificationAllOfAndroidBackgroundLayout : Channel: Push Notifications Platform: Android Allowing setting a background image for the notification. This is a JSON object containing the following keys. See our Background Image documentation for image sizes. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BasicNotificationAllOfAndroidBackgroundLayout {
    /// Asset file, android resource name, or URL to remote image.
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Title text color ARGB Hex format. Example(Blue) \"FF0000FF\".
    #[serde(rename = "headings_color", skip_serializing_if = "Option::is_none")]
    pub headings_color: Option<String>,
    /// Body text color ARGB Hex format. Example(Red) \"FFFF0000\".
    #[serde(rename = "contents_color", skip_serializing_if = "Option::is_none")]
    pub contents_color: Option<String>,
}

impl BasicNotificationAllOfAndroidBackgroundLayout {
    /// Channel: Push Notifications Platform: Android Allowing setting a background image for the notification. This is a JSON object containing the following keys. See our Background Image documentation for image sizes. 
    pub fn new() -> BasicNotificationAllOfAndroidBackgroundLayout {
        BasicNotificationAllOfAndroidBackgroundLayout {
            image: None,
            headings_color: None,
            contents_color: None,
        }
    }
}


