use email_address::EmailAddress;
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use validator::ValidationError;

use super::locale_utils::Messages;
use super::locale_utils::Namespace;

fn add_error(code: &'static str, messages: String, field_value: &str) -> ValidationError {
    ValidationError {
        code: code.into(),
        message: Some(Cow::Owned(messages)),
        params: {
            let mut params = HashMap::new();
            params.insert("value".into(), serde_json::json!(field_value));
            params
        },
    }
}

pub fn validate_name(name: &str, messages: &Messages) -> Result<(), ValidationError> {
    let mut errors = Vec::new();

    if name.trim().is_empty() {
        let msg = messages.get_str(
            Namespace::Validation,
            "name.empty",
            "Name must not be empty",
        );
        errors.push(msg);
    }

    if name.len() < 2 {
        let msg = messages.get_str(
            Namespace::Validation,
            "name.too_short",
            "Name must be at least 2 characters long",
        );
        errors.push(msg);
    }

    if name.len() > 100 {
        let msg = messages.get_str(
            Namespace::Validation,
            "name.too_long",
            "Name must be less than 100 characters",
        );
        errors.push(msg);
    }

    if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        let msg = messages.get_str(
            Namespace::Validation,
            "name.invalid_chars",
            "Name can only contain letters and spaces",
        );
        errors.push(msg);
    }

    if !errors.is_empty() {
        let concatenated_errors = errors.join(", ");
        let default_message = messages.get_str(
            Namespace::Validation,
            "name.invalid",
            &format!("The provided name is invalid ({})", concatenated_errors),
        );
        return Err(add_error("name.invalid", default_message, name));
    }

    Ok(())
}

pub fn validate_email(email: &str, messages: &Messages) -> Result<(), ValidationError> {
    let mut errors = Vec::new();

    if !is_valid_length(email) {
        let msg = messages.get_str(
            Namespace::Validation,
            "email.too_short",
            "Email must be between 5 and 254 characters",
        );
        errors.push(msg);
    }

    if !contains_at_and_dot(email) {
        let msg_at = messages.get_str(Namespace::Validation, "email.missing_at", "");
        let msg_dot = messages.get_str(Namespace::Validation, "email.missing_dot", "");
        let msg = if !msg_at.is_empty() {
            msg_at
        } else if !msg_dot.is_empty() {
            msg_dot
        } else {
            "Email must contain '@' and '.'".to_string()
        };
        errors.push(msg);
    }

    if !is_at_before_last_dot(email) {
        let msg = messages.get_str(
            Namespace::Validation,
            "email.at_before_dot",
            "The '@' must come before the last '.'",
        );
        errors.push(msg);
    }

    if contains_invalid_chars(email) {
        let msg = messages.get_str(
            Namespace::Validation,
            "invalid_chars",
            "Email must not contain spaces or non-ASCII characters",
        );
        errors.push(msg);
    }

    if contains_consecutive_dots(email) {
        let msg = messages.get_str(
            Namespace::Validation,
            "consecutive_dots",
            "Email must not contain consecutive dots",
        );
        errors.push(msg);
    }

    if starts_or_ends_with_dot(email) {
        let msg = messages.get_str(
            Namespace::Validation,
            "starts_or_ends_with_dot",
            "Email must not start or end with a dot",
        );
        errors.push(msg);
    }

    if domain_starts_with_dot(email) {
        let msg = messages.get_str(
            Namespace::Validation,
            "domain_starts_with_dot",
            "The domain part must not start with a dot",
        );
        errors.push(msg);
    }

    if let Some(domain) = get_domain(email) {
        if !is_valid_domain(domain) {
            let msg = messages.get_str(
                Namespace::Validation,
                "invalid_domain",
                "The domain part of the email is invalid",
            );
            errors.push(msg);
        }
        if !has_valid_domain_length(domain) {
            let msg = messages.get_str(
                Namespace::Validation,
                "invalid_domain_length",
                "The domain part (after '@') must have at least 2 characters before the first dot",
            );
            errors.push(msg);
        }
        if !has_valid_tld(domain) {
            let msg = messages.get_str(
                Namespace::Validation,
                "invalid_tld",
                "The TLD (after the last '.') must be at least 2 characters long and alphabetic",
            );
            errors.push(msg);
        }
    } else {
        let msg = messages.get_str(
            Namespace::Validation,
            "missing_domain",
            "Email must have a domain part after '@'",
        );
        errors.push(msg);
    }

    if errors.is_empty() && !EmailAddress::is_valid(email) {
        let msg = messages.get_str(
            Namespace::Validation,
            "invalid_format",
            "Invalid email format",
        );
        errors.push(msg);
    }

    if !errors.is_empty() {
        let concatenated_errors = errors.join(", ");
        let default_message = messages.get_str(
            Namespace::Validation,
            "invalid",
            &format!("The provided email is invalid ({})", concatenated_errors),
        );
        return Err(add_error("email.invalid", default_message, email));
    }

    Ok(())
}

fn is_valid_length(email: &str) -> bool {
    email.len() >= 5 && email.len() <= 254
}

fn contains_at_and_dot(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

fn is_at_before_last_dot(email: &str) -> bool {
    if let (Some(at), Some(dot)) = (email.find('@'), email.rfind('.')) {
        at < dot
    } else {
        false
    }
}

fn contains_invalid_chars(email: &str) -> bool {
    email.chars().any(|c| c == ' ' || !c.is_ascii())
}

fn contains_consecutive_dots(email: &str) -> bool {
    email.contains("..")
}

fn starts_or_ends_with_dot(email: &str) -> bool {
    email.starts_with('.') || email.ends_with('.')
}

fn domain_starts_with_dot(email: &str) -> bool {
    if let Some(domain) = get_domain(email) {
        domain.starts_with('.')
    } else {
        false
    }
}

fn get_domain(email: &str) -> Option<&str> {
    email.split('@').nth(1)
}

fn is_valid_domain(domain: &str) -> bool {
    !domain.is_empty() && !domain.contains(' ') && domain.contains('.')
}

fn has_valid_domain_length(domain: &str) -> bool {
    if let Some(first_dot) = domain.find('.') {
        first_dot >= 2
    } else {
        false
    }
}

fn has_valid_tld(domain: &str) -> bool {
    if let Some(last_dot) = domain.rfind('.') {
        let tld = &domain[last_dot + 1..];
        tld.len() >= 2 && tld.chars().all(|c| c.is_alphabetic())
    } else {
        false
    }
}

pub fn validate_password(password: &str, messages: &Messages) -> Result<(), ValidationError> {
    let mut errors = Vec::new();
    let mut seen_errors = HashSet::new();

    let push_error = |msg: String, errors: &mut Vec<String>, seen_errors: &mut HashSet<String>| {
        if seen_errors.insert(msg.clone()) {
            errors.push(msg);
        }
    };

    if password.len() < 8 {
        let msg = messages.get_str(
            Namespace::Validation,
            "password.too_short",
            "Password must be at least 8 characters long",
        );
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if password.contains(' ') {
        let msg = messages.get_str(
            Namespace::Validation,
            "password.contains_space",
            "Password must not contain spaces",
        );
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        let msg = messages.get_str(
            Namespace::Validation,
            "password.missing_uppercase",
            "Password must contain at least one uppercase letter",
        );
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        let msg = messages.get_str(
            Namespace::Validation,
            "password.missing_lowercase",
            "Password must contain at least one lowercase letter",
        );
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        let msg = messages.get_str(
            Namespace::Validation,
            "password.missing_digit",
            "Password must contain at least one digit",
        );
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if !password.chars().any(|c| !c.is_alphanumeric()) {
        let msg = messages.get_str(
            Namespace::Validation,
            "password.missing_special_char",
            "Password must contain at least one special character",
        );
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        let concatenated_errors = errors.join(", ");
        let default_message = messages.get_str(
            Namespace::Validation,
            "password.invalid",
            &format!("The provided password is invalid ({})", concatenated_errors),
        );
        Err(add_error("password.invalid", default_message, password))
    }
}
