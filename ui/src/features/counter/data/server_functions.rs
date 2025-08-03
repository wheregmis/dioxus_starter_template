use dioxus::prelude::*;

#[server(GetCounter)]
pub async fn get_counter() -> ServerFnResult<i32> {
    use crate::backend_ext::req_parts_counter::Counter;
    let counter = extract::<Counter, _>()
        .await
        .map_err(|_| ServerFnError::ServerError("Failed to extract counter".to_string()))?;
    Ok(counter.0 as i32)
}
