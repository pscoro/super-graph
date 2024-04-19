use crate::utils::Error;

/// Wrapper for Result<(), Box<dyn Error>>
pub type EmptyResult = Result<(), Box<dyn Error>>;
