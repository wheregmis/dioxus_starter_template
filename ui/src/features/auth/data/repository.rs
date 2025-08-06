use crate::features::{User, auth::domain::AuthRepository};

pub struct AuthRepositoryImpl;

impl AuthRepository for AuthRepositoryImpl {
    async fn get_current_user() -> Result<User, String> {
        use crate::features::auth::data::server_functions::get_current_user;
        match get_current_user().await {
            Ok(user) => Ok(user),
            Err(e) => Err(e.to_string()),
        }
    }
}
