use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("{error_name:?}:{error_details:?}")]
    __Error {
        error_name: String,
        error_details: String,
    },
}

#[derive(TokenizerError, Debug)]
pub enum IllegalCharError {
    error_name = "IllegalCharError",
}
