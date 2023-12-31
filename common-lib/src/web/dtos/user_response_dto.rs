use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct UserResponseDto {
    pub user_id: Option<Uuid>,
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub date_time_created: Option<DateTime<Utc>>,
    pub date_time_updated: Option<DateTime<Utc>>,
}
