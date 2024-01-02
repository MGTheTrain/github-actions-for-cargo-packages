use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct User {
    #[validate(required)]
    pub user_id: Option<Uuid>,

    #[validate(required, length(max = 50))]
    pub user_name: Option<String>,

    #[validate(required, length(min = 10))]
    pub user_password: Option<String>,

    #[validate(required)]
    pub email: Option<String>,

    #[validate(required)]
    pub date_time_created: Option<DateTime<Utc>>,

    #[validate(required)]
    pub date_time_updated: Option<DateTime<Utc>>,
}

// Custom validation function for Uuid
fn validate_uuid(uuid: &Option<Uuid>) -> Result<(), ValidationError> {
    if let Some(uid) = uuid {
        if uid.is_nil() {
            return Err(ValidationError::new("User ID cannot be nil"));
        }
    }
    Ok(())
}

// Prefer having tests decoupled from implementation for less file content
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_user_validation() {
//         let user_id = Some(Uuid::new_v4());
//         let user = User {
//             user_id,
//             user_name: Some("JohnDoe".to_string()),
//             user_password: Some("password123".to_string()),
//             email: Some("johndoe@example.com".to_string()),
//             date_time_created: Some(Utc::now()),
//             date_time_updated: Some(Utc::now()),
//         };

//         let validation_result = user.validate();
//         match validation_result {
//             Ok(_) => {
//                 println!("Validation succeeded!");
//             }
//             Err(errors) => {
//                 println!("Validation errors: {:?}", errors);
//             }
//         }
//     }
// }
