use crate::validation::validate_plan;

pub struct ActivateResponse {
    pub status: u16,
    pub body_error: Option<String>,
    pub body_status: Option<String>,
}

pub async fn activate_handler(plan: String) -> ActivateResponse {
    match validate_plan(&plan) {
        Ok(()) => ActivateResponse {
            status: 200,
            body_error: None,
            body_status: Some("ACTIVE".to_string()),
        },
        Err(code) => ActivateResponse {
            status: 422,
            body_error: Some(code.to_string()),
            body_status: None,
        },
    }
}
