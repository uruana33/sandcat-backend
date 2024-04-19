use abi::errors::Error;
use abi::message::{User, UserUpdate, UserWithMatchType};
use async_trait::async_trait;
use std::fmt::Debug;

#[async_trait]
pub trait UserRepo: Sync + Send + Debug {
    /// create user
    async fn create_user(&self, user: User) -> Result<User, Error>;

    /// get user by id
    async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, Error>;

    /// search user by pattern, return users and matched method
    async fn search_user(
        &self,
        user_id: &str,
        pattern: &str,
    ) -> Result<Option<UserWithMatchType>, Error>;

    async fn update_user(&self, user: UserUpdate) -> Result<User, Error>;

    async fn verify_pwd(&self, account: &str, password: &str) -> Result<Option<User>, Error>;
}
