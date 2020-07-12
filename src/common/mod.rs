use std::error::Error;

pub mod helpers;

pub fn map_dumming_error(e: impl Error) -> String {
    format!("error code: {}", e)
}
