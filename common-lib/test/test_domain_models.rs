use validator::{Validate, ValidationError};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use common_lib::domain::models::user::User;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_validation() {
        let user_id = Some(Uuid::new_v4());
        let user = User {
            user_id,
            user_name: Some("JohnDoe".to_string()),
            user_password: Some("password123".to_string()),
            email: Some("johndoe@example.com".to_string()),
            date_time_created: Some(Utc::now()),
            date_time_updated: Some(Utc::now()),
        };

        let validation_result = user.validate();
        match validation_result {
            Ok(_) => {
                println!("Validation succeeded!");
            }
            Err(errors) => {
                println!("Validation errors: {:?}", errors);
            }
        }
    }
}