use email_address::EmailAddress;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use validator::ValidationError;

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

pub fn validate_name(name: &str, messages: &Value) -> Result<(), ValidationError> {
    let mut errors = Vec::new();

    if name.trim().is_empty() {
        let msg = messages
            .get("name.empty")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Name must not be empty".to_string());
        errors.push(msg);
    }

    if name.len() < 2 {
        let msg = messages
            .get("name.too_short")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Name must be at least 2 characters long".to_string());
        errors.push(msg);
    }

    if name.len() > 100 {
        let msg = messages
            .get("name.too_long")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Name must be less than 100 characters".to_string());
        errors.push(msg);
    }

    if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) {
        let msg = messages
            .get("name.invalid_chars")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Name can only contain letters and spaces".to_string());
        errors.push(msg);
    }

    if !errors.is_empty() {
        let concatenated_errors = errors.join(", ");
        let default_message = messages
            .get("name.invalid")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| format!("The provided name is invalid ({})", concatenated_errors));
        return Err(add_error("name.invalid", default_message, name));
    }

    Ok(())
}

pub fn validate_email(email: &str, messages: &Value) -> Result<(), ValidationError> {
    let mut errors = Vec::new();

    if !is_valid_length(email) {
        let msg = messages
            .get("email.too_short")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Email must be between 5 and 254 characters".to_string());
        errors.push(msg);
    }

    if !contains_at_and_dot(email) {
        let msg_at = messages
            .get("email.missing_at")
            .and_then(Value::as_str)
            .map(String::from);
        let msg_dot = messages
            .get("email.missing_dot")
            .and_then(Value::as_str)
            .map(String::from);
        let msg = msg_at
            .or(msg_dot)
            .unwrap_or_else(|| "Email must contain '@' and '.'".to_string());
        errors.push(msg);
    }

    if !is_at_before_last_dot(email) {
        let msg = messages
            .get("email.at_before_dot")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "The '@' must come before the last '.'".to_string());
        errors.push(msg);
    }

    if contains_invalid_chars(email) {
        let msg = messages
            .get("email.invalid_chars")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Email must not contain spaces or non-ASCII characters".to_string());
        errors.push(msg);
    }

    if contains_consecutive_dots(email) {
        let msg = messages
            .get("email.consecutive_dots")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Email must not contain consecutive dots".to_string());
        errors.push(msg);
    }

    if starts_or_ends_with_dot(email) {
        let msg = messages
            .get("email.starts_or_ends_with_dot")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Email must not start or end with a dot".to_string());
        errors.push(msg);
    }

    if domain_starts_with_dot(email) {
        let msg = messages
            .get("email.domain_starts_with_dot")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "The domain part must not start with a dot".to_string());
        errors.push(msg);
    }

    if let Some(domain) = get_domain(email) {
        if !is_valid_domain(domain) {
            let msg = messages
                .get("email.invalid_domain")
                .and_then(Value::as_str)
                .map(String::from)
                .unwrap_or_else(|| "The domain part of the email is invalid".to_string());
            errors.push(msg);
        }
        if !has_valid_domain_length(domain) {
            let msg = messages
                .get("email.invalid_domain_length")
                .and_then(Value::as_str)
                .map(String::from)
                .unwrap_or_else(|| {
                    "The domain part (after '@') must have at least 2 characters before the first dot".to_string()
                });
            errors.push(msg);
        }
        if !has_valid_tld(domain) {
            let msg = messages
                .get("email.invalid_tld")
                .and_then(Value::as_str)
                .map(String::from)
                .unwrap_or_else(|| {
                    "The TLD (after the last '.') must be at least 2 characters long and alphabetic"
                        .to_string()
                });
            errors.push(msg);
        }
    } else {
        let msg = messages
            .get("email.missing_domain")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Email must have a domain part after '@'".to_string());
        errors.push(msg);
    }

    if errors.is_empty() && !EmailAddress::is_valid(email) {
        let msg = messages
            .get("email.invalid_format")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Invalid email format".to_string());
        errors.push(msg);
    }

    if !errors.is_empty() {
        let concatenated_errors = errors.join(", ");
        let default_message = messages
            .get("email.invalid")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| format!("The provided email is invalid ({})", concatenated_errors));
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

pub fn validate_password(password: &str, messages: &Value) -> Result<(), ValidationError> {
    let mut errors = Vec::new();
    let mut seen_errors = HashSet::new();

    let push_error = |msg: String, errors: &mut Vec<String>, seen_errors: &mut HashSet<String>| {
        if seen_errors.insert(msg.clone()) {
            errors.push(msg);
        }
    };

    if password.len() < 8 {
        let msg = messages
            .get("password.too_short")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Password must be at least 8 characters long".to_string());
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if password.contains(' ') {
        let msg = messages
            .get("password.contains_space")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Password must not contain spaces".to_string());
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        let msg = messages
            .get("password.missing_uppercase")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Password must contain at least one uppercase letter".to_string());
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        let msg = messages
            .get("password.missing_lowercase")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Password must contain at least one lowercase letter".to_string());
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        let msg = messages
            .get("password.missing_digit")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Password must contain at least one digit".to_string());
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if !password.chars().any(|c| !c.is_alphanumeric()) {
        let msg = messages
            .get("password.missing_special_char")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| "Password must contain at least one special character".to_string());
        push_error(msg, &mut errors, &mut seen_errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        let concatenated_errors = errors.join(", ");
        let default_message = messages
            .get("password.invalid")
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| {
                format!("The provided password is invalid ({})", concatenated_errors)
            });
        Err(add_error("password.invalid", default_message, password))
    }
}
