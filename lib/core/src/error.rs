use thiserror::Error as ThisError;

#[derive(ThisError, Debug, Clone)]
pub enum Error {
    #[error("Cannot publish to the message attribute topic")]
    MessageAttributePublishError(String),
    #[error("Cannot subscribe to the message attribute topic")]
    MessageAttributeSubscribeError(String),
    #[error("Internal weak pointer cannot be upgraded")]
    InternalPointerUpgrade,
    #[error("Internal logic lead to this error")]
    InternalLogic(String),
    #[error("Error when trying to spawn a task")]
    Spawn(String),
    #[error("One of the provided settings is wrong")]
    BadSettings(String),
    #[error("Error during serialization")]
    SerializeFailure(String),
    #[error("Error during deserialization")]
    DeserializeFailure(String),
    #[error("Error")]
    Generic(String),
    #[error("We just don't know what happened")]
    Wtf,
}
