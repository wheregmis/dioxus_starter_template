use axum::extract::FromRequestParts;
use http::request::Parts;
use serde::{Deserialize, Serialize};

const COUNTER_KEY: &str = "counter";

#[derive(Default, Deserialize, Serialize)]
pub struct Counter(pub usize);

impl<S> FromRequestParts<S> for Counter
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        use tower_sessions::Session;

        let session = Session::from_request_parts(req, state).await?;
        let counter: Counter = session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();
        session.insert(COUNTER_KEY, counter.0 + 4).await.unwrap();
        Ok(counter)
    }
}
