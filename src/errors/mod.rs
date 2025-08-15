pub mod errors;

pub use errors::FTVError;
pub type AppResult<T> = Result<T, FTVError>;
