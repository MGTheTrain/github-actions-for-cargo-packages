use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct UserRequestDto {
    #[validate(required, length(max = 50))]
    pub user_name: Option<String>,

    #[validate(required, length(min = 10))]
    pub user_password: Option<String>,

    #[validate(required)]
    pub email: Option<String>,
}

// Prefer having tests decoupled from implementation for less file content
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_user_request_dto_validation() {
//         let userRequestDto = UserRequestDto {
//             user_name: Some("JohnDoe".to_string()),
//             user_password: Some("password123".to_string()),
//             email: Some("johndoe@example.com".to_string()),
//         };

//         let validation_result = userRequestDto.validate();
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
