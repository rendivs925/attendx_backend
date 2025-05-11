use crate::utils::locale_utils::Messages;

pub type ValidationResult = Result<(), String>;

#[derive(Clone, Copy)]
pub struct ValidationFn(pub fn(&str, &Messages) -> ValidationResult);

unsafe impl Send for ValidationFn {}
unsafe impl Sync for ValidationFn {}
