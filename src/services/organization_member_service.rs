use crate::repositories::user_repository::UserRepository;
use std::sync::Arc;

pub struct OrganizationMemberService {
    pub user_repository: Arc<UserRepository>,
}

impl OrganizationMemberService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }
}
