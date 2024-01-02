use chrono::{DateTime, Utc};
use common_lib::web::dtos::user_request_dto::UserRequestDto;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_request_dto_validation() {
        let userRequestDto = UserRequestDto {
            user_name: Some("JohnDoe".to_string()),
            user_password: Some("password123".to_string()),
            email: Some("johndoe@example.com".to_string()),
        };

        let validation_result = userRequestDto.validate();
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
