use std::{fmt::Display, io::Error as IoError};

use nvim_oxi::{
    api::{err_writeln, Error as OxiApiError},
    Error as OxiError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[error("Neovim API error: {0}")]
    Api(#[from] OxiApiError),

    #[error("Custom error: {0}")]
    Custom(String),
}

impl From<PluginError> for OxiError {
    fn from(err: PluginError) -> Self {
        err_writeln(&format!("{}", err));
        OxiError::Api(OxiApiError::Other(format!("{}", err)))
    }
}

pub fn handle_error<T, E>(result: Result<T, E>, context: &str) -> Result<T, PluginError>
where
    E: Display,
{
    result.map_err(|e| {
        let err_msg = format!("{}: {}", context, e);
        PluginError::Custom(err_msg)
    })
}
