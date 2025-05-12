use rayon::prelude::*;
use validator::ValidationError;

use crate::utils::{locale_utils::Messages, validation_utils::add_error};

const MIN_NAME_LENGTH: usize = 2;
const MAX_NAME_LENGTH: usize = 100;

fn is_not_empty(name: &str, messages: &Messages) -> Result<(), String> {
    if name.trim().is_empty() {
        Err(messages.get_validation_message("name.empty", "Name must not be empty"))
    } else {
        Ok(())
    }
}

fn has_min_length(name: &str, messages: &Messages) -> Result<(), String> {
    if name.len() < MIN_NAME_LENGTH {
        Err(messages.get_validation_message(
            "name.too_short",
            &format!("Name must be at least {} characters long", MIN_NAME_LENGTH),
        ))
    } else {
        Ok(())
    }
}

fn has_max_length(name: &str, messages: &Messages) -> Result<(), String> {
    if name.len() > MAX_NAME_LENGTH {
        Err(messages.get_validation_message(
            "name.too_long",
            &format!("Name must be less than {} characters", MAX_NAME_LENGTH),
        ))
    } else {
        Ok(())
    }
}

fn has_valid_chars(name: &str, messages: &Messages) -> Result<(), String> {
    if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        Err(messages.get_validation_message(
            "name.invalid_chars",
            "Name can only contain letters and spaces",
        ))
    } else {
        Ok(())
    }
}

pub fn validate_name(name: &str, messages: &Messages) -> Result<(), ValidationError> {
    let validations = [
        is_not_empty,
        has_min_length,
        has_max_length,
        has_valid_chars,
    ];

    let errors: Vec<String> = validations
        .par_iter()
        .filter_map(|f| f(name, messages).err())
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        let concatenated_errors = errors.join(", ");
        Err(add_error("name.invalid", concatenated_errors, name))
    }
}
