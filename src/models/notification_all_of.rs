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
pub struct NotificationAllOf {
    /// Channel: All Schedule notification for future delivery. API defaults to UTC -1100 Examples: All examples are the exact same date & time. \"Thu Sep 24 2015 14:00:00 GMT-0700 (PDT)\" \"September 24th 2015, 2:00:00 pm UTC-07:00\" \"2015-09-24 14:00:00 GMT-0700\" \"Sept 24 2015 14:00:00 GMT-0700\" \"Thu Sep 24 2015 14:00:00 GMT-0700 (Pacific Daylight Time)\" Note: SMS currently only supports send_after parameter. 
    #[serde(rename = "send_after", skip_serializing_if = "Option::is_none")]
    pub send_after: Option<String>,
}

impl NotificationAllOf {
    pub fn new() -> NotificationAllOf {
        NotificationAllOf {
            send_after: None,
        }
    }
}


