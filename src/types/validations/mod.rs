use crate::utils::locale_utils::Messages;

pub type ValidationResult = Result<(), String>;

pub type ValidationFn = fn(&str, &Messages) -> ValidationResult;
