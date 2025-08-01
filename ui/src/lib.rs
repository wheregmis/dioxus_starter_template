#[cfg(feature = "server")]
use axum::extract::FromRequestParts;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use http::request::Parts;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("./assets/favicon.ico");
const MAIN_CSS: Asset = asset!("./assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let current_counter = use_resource(move || async move { get_counter().await });

    rsx! {
        match &*current_counter.read_unchecked() {
            Some(Ok(counter)) => rsx! { div { "Counter: {counter}" } },
            Some(Err(e)) => rsx! { p { "Loading counter failed, {e}" } },
            None =>  rsx! { p { "Loading..." } }
        }
    }
}

#[cfg(feature = "server")]
const COUNTER_KEY: &str = "counter";

#[cfg(feature = "server")]
#[derive(Default, Deserialize, Serialize)]
struct Counter(usize);

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for Counter
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        use tower_sessions::Session;

        let session = Session::from_request_parts(req, state).await?;
        let counter: Counter = session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();
        session.insert(COUNTER_KEY, counter.0 + 1).await.unwrap();
        Ok(counter)
    }
}

#[server(GetCounter)]
async fn get_counter() -> ServerFnResult<i32> {
    let counter = extract::<Counter, _>()
        .await
        .map_err(|_| ServerFnError::ServerError("Failed to extract counter".to_string()))?;
    Ok(counter.0 as i32)
}
