use async_trait::async_trait;

use crate::error::Error;


#[async_trait]
pub trait QueryCrud {
    async fn create(&self) -> Result<(), Error>;
    async fn read(&self) -> Result<(), Error>;
    async fn update(&self) -> Result<(), Error>;
    async fn delete(&self) -> Result<(), Error>;
}