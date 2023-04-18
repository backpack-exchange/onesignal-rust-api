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
pub struct Player {
    /// The device's OneSignal ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// If true, this is the equivalent of a user being Unsubscribed
    #[serde(rename = "invalid_identifier", skip_serializing_if = "Option::is_none")]
    pub invalid_identifier: Option<bool>,
    #[serde(rename = "app_id", skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Required The device's platform:   0 = iOS   1 = Android   2 = Amazon   3 = WindowsPhone (MPNS)   4 = Chrome Apps / Extensions   5 = Chrome Web Push   6 = Windows (WNS)   7 = Safari   8 = Firefox   9 = MacOS   10 = Alexa   11 = Email   13 = For Huawei App Gallery Builds SDK Setup. Not for Huawei Devices using FCM   14 = SMS 
    #[serde(rename = "device_type")]
    pub device_type: i32,
    /// a custom user ID
    #[serde(rename = "external_user_id", skip_serializing_if = "Option::is_none")]
    pub external_user_id: Option<String>,
    /// Only required if you have enabled Identity Verification and device_type is NOT 11 email.
    #[serde(rename = "external_user_id_auth_hash", skip_serializing_if = "Option::is_none")]
    pub external_user_id_auth_hash: Option<String>,
    /// Email - Only required if you have enabled Identity Verification and device_type is email (11).
    #[serde(rename = "email_auth_hash", skip_serializing_if = "Option::is_none")]
    pub email_auth_hash: Option<String>,
    /// Recommended: For Push Notifications, this is the Push Token Identifier from Google or Apple. For Apple Push identifiers, you must strip all non alphanumeric characters. Examples: iOS: 7abcd558f29d0b1f048083e2834ad8ea4b3d87d8ad9c088b33c132706ff445f0 Android: APA91bHbYHk7aq-Uam_2pyJ2qbZvqllyyh2wjfPRaw5gLEX2SUlQBRvOc6sck1sa7H7nGeLNlDco8lXj83HWWwzV... For Email Addresses, set the full email address email@email.com and make sure to set device_type to 11. 
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Language code. Typically lower case two letters, except for Chinese where it must be one of zh-Hans or zh-Hant. Example: en 
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Number of seconds away from UTC. Example: -28800 
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<i32>,
    /// Version of your app. Example: 1.1 
    #[serde(rename = "game_version", skip_serializing_if = "Option::is_none")]
    pub game_version: Option<String>,
    /// Device make and model. Example: iPhone5,1 
    #[serde(rename = "device_model", skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    /// Device operating system version. Example: 7.0.4 
    #[serde(rename = "device_os", skip_serializing_if = "Option::is_none")]
    pub device_os: Option<String>,
    /// The ad id for the device's platform: Android = Advertising Id iOS = identifierForVendor WP8.0 = DeviceUniqueId WP8.1 = AdvertisingId 
    #[serde(rename = "ad_id", skip_serializing_if = "Option::is_none")]
    pub ad_id: Option<String>,
    /// Name and version of the sdk/plugin that's calling this API method (if any)
    #[serde(rename = "sdk", skip_serializing_if = "Option::is_none")]
    pub sdk: Option<String>,
    /// Number of times the user has played the game, defaults to 1
    #[serde(rename = "session_count", skip_serializing_if = "Option::is_none")]
    pub session_count: Option<i32>,
    /// Custom tags for the player. Only support string and integer key value pairs. Does not support arrays or other nested objects. Setting a tag value to null or an empty string will remove the tag. Example: {\"foo\":\"bar\",\"this\":\"that\"} Limitations: - 100 tags per call - Android SDK users: tags cannot be removed or changed via API if set through SDK sendTag methods. Recommended to only tag devices with 1 kilobyte of data Please consider using your own Database to save more than 1 kilobyte of data. See: Internal Database & CRM 
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    /// Amount the user has spent in USD, up to two decimal places
    #[serde(rename = "amount_spent", skip_serializing_if = "Option::is_none")]
    pub amount_spent: Option<f32>,
    /// Unixtime when the player joined the game
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    /// Seconds player was running your app.
    #[serde(rename = "playtime", skip_serializing_if = "Option::is_none")]
    pub playtime: Option<i64>,
    /// Current iOS badge count displayed on the app icon NOTE: Not supported for apps created after June 2018, since badge count for apps created after this date are handled on the client. 
    #[serde(rename = "badge_count", skip_serializing_if = "Option::is_none")]
    pub badge_count: Option<i32>,
    /// Unixtime when the player was last active
    #[serde(rename = "last_active", skip_serializing_if = "Option::is_none")]
    pub last_active: Option<i32>,
    /// 1 = subscribed -2 = unsubscribed iOS - These values are set each time the user opens the app from the SDK. Use the SDK function set Subscription instead. Android - You may set this but you can no longer use the SDK method setSubscription later in your app as it will create synchronization issues. 
    #[serde(rename = "notification_types", skip_serializing_if = "Option::is_none")]
    pub notification_types: Option<i32>,
    /// This is used in deciding whether to use your iOS Sandbox or Production push certificate when sending a push when both have been uploaded. Set to the iOS provisioning profile that was used to build your app. 1 = Development 2 = Ad-Hoc Omit this field for App Store builds. 
    #[serde(rename = "test_type", skip_serializing_if = "Option::is_none")]
    pub test_type: Option<i32>,
    /// Longitude of the device, used for geotagging to segment on.
    #[serde(rename = "long", skip_serializing_if = "Option::is_none")]
    pub long: Option<f32>,
    /// Latitude of the device, used for geotagging to segment on.
    #[serde(rename = "lat", skip_serializing_if = "Option::is_none")]
    pub lat: Option<f32>,
    /// Country code in the ISO 3166-1 Alpha 2 format
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl Player {
    pub fn new(device_type: i32) -> Player {
        Player {
            id: None,
            invalid_identifier: None,
            app_id: None,
            device_type,
            external_user_id: None,
            external_user_id_auth_hash: None,
            email_auth_hash: None,
            identifier: None,
            language: None,
            timezone: None,
            game_version: None,
            device_model: None,
            device_os: None,
            ad_id: None,
            sdk: None,
            session_count: None,
            tags: None,
            amount_spent: None,
            created_at: None,
            playtime: None,
            badge_count: None,
            last_active: None,
            notification_types: None,
            test_type: None,
            long: None,
            lat: None,
            country: None,
        }
    }
}


