/*
 * OneSignal
 *
 * A powerful way to send personalized messages at scale and build effective customer engagement strategies. Learn more at onesignal.com
 *
 * The version of the OpenAPI document: 1.2.2
 * Contact: devrel@onesignal.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SegmentNotificationTarget {
    /// The segment names you want to target. Users in these segments will receive a notification. This targeting parameter is only compatible with excluded_segments. Example: [\"Active Users\", \"Inactive Users\"] 
    #[serde(rename = "included_segments", skip_serializing_if = "Option::is_none")]
    pub included_segments: Option<Vec<String>>,
    /// Segment that will be excluded when sending. Users in these segments will not receive a notification, even if they were included in included_segments. This targeting parameter is only compatible with included_segments. Example: [\"Active Users\", \"Inactive Users\"] 
    #[serde(rename = "excluded_segments", skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<String>>,
}

impl SegmentNotificationTarget {
    pub fn new() -> SegmentNotificationTarget {
        SegmentNotificationTarget {
            included_segments: None,
            excluded_segments: None,
        }
    }
}


