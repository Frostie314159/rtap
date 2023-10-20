use thiserror::Error;

#[derive(Clone, Copy, Debug, Error)]
pub enum RadiotapError {
    #[error("Less header data was encoutered, than was expected")]
    HeaderIncomplete,
    #[error("The version field isn't zero")]
    VersionIsNotZero,
    #[error("Less bytes than specified in header")]
    UnderlyingIterEndedEarly,
    #[error("skip_length field is too short")]
    SkipLenTooShort,
    #[error("Unknown")]
    Unknown,
}
