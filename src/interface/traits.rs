


#[async_trait]
pub trait IdentityProvider : Send {
    fn get_info(&self) -> Value;
}
