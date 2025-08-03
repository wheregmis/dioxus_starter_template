use super::entity::Counter;
use crate::features::counter::data::CounterRepositoryImpl;

pub trait CounterRepository {
    fn get_counter() -> impl std::future::Future<Output = Result<Counter, String>> + Send;
}

pub struct GetCounterUseCase;

impl GetCounterUseCase {
    pub async fn execute() -> Result<Counter, String> {
        CounterRepositoryImpl::get_counter().await
    }
}
