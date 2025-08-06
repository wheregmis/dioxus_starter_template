use dioxus::prelude::*;

#[server(
      name = GetCounter,
       prefix = "/api",
       endpoint = "get_counter"
     )]
pub async fn get_counter() -> ServerFnResult<i32> {
    use crate::features::counter::server_ext::Counter;
    let counter = extract::<Counter, _>()
        .await
        .map_err(|_| ServerFnError::ServerError("Failed to extract counter".to_string()))?;
    Ok(counter.0 as i32)
}
