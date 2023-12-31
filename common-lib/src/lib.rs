pub mod domain {
    pub mod models {
        pub mod user;
    }
}

pub mod web {
    pub mod dtos {
        pub mod user_request_dto;
        pub mod user_response_dto;
    }
}

// Some integration tests utilizing several modules