use actix_web::HttpResponse;
use serde_json::json;
use std::{borrow::Cow, collections::HashMap};
use validator::{ValidationError, ValidationErrors};

use crate::{
    types::requests::auth::{login_request::LoginRequest, register_request::RegisterRequest},
    types::responses::api_response::{ApiResponse, ErrorDetails},
    utils::locale_utils::Messages,
    validations::{email::validate_email, name::validate_name, password::validate_password},
};

pub fn handle_validation_error(errors: ValidationErrors, msg: &str) -> HttpResponse {
    let error_details = ErrorDetails {
        details: Some(json!(&errors)),
    };
    HttpResponse::BadRequest().json(ApiResponse::<()>::error(msg, error_details))
}

pub fn handle_internal_error(err: impl ToString) -> HttpResponse {
    let error_details = ErrorDetails { details: None };
    HttpResponse::InternalServerError()
        .json(ApiResponse::<()>::error(err.to_string(), error_details))
}

pub fn validate_register_data(
    data: &RegisterRequest,
    messages: &Messages,
) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    if let Err(e) = validate_name(&data.name, messages) {
        errors.add("name", e);
    }
    if let Err(e) = validate_email(&data.email, messages) {
        errors.add("email", e);
    }
    if let Err(e) = validate_password(&data.password, messages) {
        errors.add("password", e);
    }

    if errors.errors().is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_login_data(
    data: &LoginRequest,
    messages: &Messages,
) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    if let Err(e) = validate_email(&data.email, messages) {
        errors.add("email", e);
    }
    if let Err(e) = validate_password(&data.password, messages) {
        errors.add("password", e);
    }

    if errors.errors().is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn add_error(code: &'static str, message: String, field_value: &str) -> ValidationError {
    ValidationError {
        code: code.into(),
        message: Some(Cow::Owned(message)),
        params: {
            let mut params = HashMap::new();
            params.insert("value".into(), json!(field_value));
            params
        },
    }
}
