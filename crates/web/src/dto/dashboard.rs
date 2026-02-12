use serde::Serialize;

#[derive(Debug, Clone, Serialize, bon::Builder)]
#[serde(rename_all = "camelCase")]
pub struct DashboardAnalyticsProps {
    pub total_conversations: u64,
    pub total_messages: u64,
    pub total_phone_numbers: u64,
}
