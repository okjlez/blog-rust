use async_trait::async_trait;

use crate::error::Error;


#[async_trait]
pub trait QueryCud {
    async fn create(&self) -> Result<(), Error>;
    async fn update<F, V>(&self, field: F, value: V) -> Result<(), Error>;
    async fn delete(&self) -> Result<(), Error>;
}