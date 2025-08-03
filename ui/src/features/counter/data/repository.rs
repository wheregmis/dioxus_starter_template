use crate::features::counter::domain::{Counter, CounterRepository};

use crate::features::counter::data::server_functions::get_counter;

pub struct CounterRepositoryImpl;

impl CounterRepository for CounterRepositoryImpl {
    async fn get_counter() -> Result<Counter, String> {
        match get_counter().await {
            Ok(value) => Ok(Counter::new(value)),
            Err(e) => Err(format!("Failed to get counter: {e:?}")),
        }
    }
}
