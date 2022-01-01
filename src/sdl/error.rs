use std::{
    error::Error,
    fmt::{self},
};

use sdl2::{video::WindowBuildError, IntegerOrSdlError};

#[derive(Debug)]
pub enum SdlError {
    ContextError(String),
    CanvasError(IntegerOrSdlError),
    WindowError(WindowError),
    KeyboardError(String),
}

impl fmt::Display for SdlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            SdlError::CanvasError(error) => {
                let mut message = "canvas error: ".to_owned();
                message.push_str(&error.to_string());
                message
            }
            SdlError::ContextError(error) => {
                let mut message = "context error: ".to_owned();
                message.push_str(&error.to_string());
                message
            }
            SdlError::WindowError(error) => {
                let mut message = "window error: ".to_owned();
                message.push_str(&error.to_string());
                message
            }
            SdlError::KeyboardError(error) => {
                let mut message = "keyboard error: ".to_owned();
                message.push_str(&error.to_string());
                message
            }
        };

        write!(f, "{}", error)
    }
}

impl Error for SdlError {}

impl From<String> for SdlError {
    fn from(error: String) -> Self {
        SdlError::ContextError(error)
    }
}

impl From<IntegerOrSdlError> for SdlError {
    fn from(error: IntegerOrSdlError) -> Self {
        SdlError::CanvasError(error)
    }
}

impl From<WindowError> for SdlError {
    fn from(error: WindowError) -> Self {
        SdlError::WindowError(error)
    }
}

#[derive(Debug)]
pub enum WindowError {
    String(String),
    WindowBuildError(WindowBuildError),
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            WindowError::String(s) => s.to_owned(),
            WindowError::WindowBuildError(error) => error.to_string(),
        };

        write!(f, "{}", error)
    }
}

impl Error for WindowError {}

impl From<String> for WindowError {
    fn from(error: String) -> Self {
        WindowError::String(error)
    }
}

impl From<WindowBuildError> for WindowError {
    fn from(error: WindowBuildError) -> Self {
        WindowError::WindowBuildError(error)
    }
}
