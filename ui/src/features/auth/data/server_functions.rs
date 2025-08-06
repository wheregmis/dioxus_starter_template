use crate::features::{
    User,
    auth::domain::{AuthRequest, AuthResponse},
};
use dioxus::prelude::*;

#[server(GetCurrentUser)]
pub async fn get_current_user() -> ServerFnResult<User> {
    use torii_axum::AuthUser;
    let user = extract::<AuthUser, _>()
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(crate::features::auth::domain::User {
        id: user.0.id.to_string(),
        email: user.0.email,
    })
}
