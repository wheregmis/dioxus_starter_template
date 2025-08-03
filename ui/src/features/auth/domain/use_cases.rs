use super::entity::{AuthRequest, AuthResponse, User};

pub trait AuthRepository {
    fn get_current_user() -> impl std::future::Future<Output = Result<User, String>> + Send;
}

// Lets create a use case for getting the current user
pub struct GetCurrentUserUseCase;

impl GetCurrentUserUseCase {
    pub async fn execute() -> Result<User, String> {
        crate::features::auth::data::AuthRepositoryImpl::get_current_user().await
    }
}
