use actix_web::HttpResponse;
use rayon::prelude::*;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::{borrow::Cow, collections::HashMap};
use validator::{ValidationError, ValidationErrors};

use crate::{
    types::requests::auth::{login_request::LoginRequest, register_request::RegisterRequest},
    types::responses::api_response::{ApiResponse, ErrorDetails},
    utils::locale_utils::Messages,
    validations::{email::validate_email, name::validate_name, password::validate_password},
};

type FieldValidation<'a> = (
    &'static str,
    &'a str,
    fn(&'a str, &Messages) -> Result<(), ValidationError>,
);

pub fn validate_fields(
    fields: Vec<FieldValidation>,
    messages: &Messages,
) -> Result<(), ValidationErrors> {
    let errors = Arc::new(Mutex::new(ValidationErrors::new()));

    fields.par_iter().for_each(|(field, value, validator)| {
        if let Err(error) = validator(value, messages) {
            let mut errors_lock = errors.lock().unwrap();
            errors_lock.add(field, error);
        }
    });

    let errors_lock = errors.lock().unwrap();
    if errors_lock.errors().is_empty() {
        Ok(())
    } else {
        Err(errors_lock.clone())
    }
}

pub fn handle_validation_error(errors: ValidationErrors, msg: &str) -> HttpResponse {
    let error_details = ErrorDetails {
        details: Some(json!(&errors)),
    };
    HttpResponse::BadRequest().json(ApiResponse::<()>::error(msg, Some(error_details)))
}

pub fn handle_internal_error(err: impl ToString) -> HttpResponse {
    HttpResponse::InternalServerError().json(ApiResponse::<()>::error(err.to_string(), None))
}

pub fn validate_register_data(
    data: &RegisterRequest,
    messages: &Messages,
) -> Result<(), ValidationErrors> {
    validate_fields(
        vec![
            ("name", &data.name, validate_name),
            ("email", &data.email, validate_email),
            ("password", &data.password, validate_password),
        ],
        messages,
    )
}

pub fn validate_login_data(
    data: &LoginRequest,
    messages: &Messages,
) -> Result<(), ValidationErrors> {
    validate_fields(
        vec![
            ("email", &data.email, validate_email),
            ("password", &data.password, validate_password),
        ],
        messages,
    )
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
