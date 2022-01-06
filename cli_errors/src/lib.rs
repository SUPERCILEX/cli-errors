use anyhow::{Error, Result};
use exitcode::ExitCode;

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
