use anyhow::{Error, Result};
use exitcode::ExitCode;

pub use cli_errors_macros::main;

pub type CliResult<T> = Result<T, CliExitError>;

#[derive(Debug)]
pub struct CliExitError {
    pub code: ExitCode,
    pub source: Option<Error>,
}

pub trait CliExitAnyhowWrapper<T> {
    fn with_code(self, error_code: i32) -> CliResult<T>;
}

impl<T> CliExitAnyhowWrapper<T> for Result<T> {
    fn with_code(self, error_code: i32) -> CliResult<T> {
        self.map_err(|e| CliExitError {
            code: error_code,
            source: Some(e),
        })
    }
}

impl From<Error> for CliExitError {
    fn from(e: Error) -> Self {
        Self {
            code: 1,
            source: Some(e),
        }
    }
}
