#[macro_use]
extern crate log;

mod errors;
mod logging;

pub use errors::{Error, ErrorMessage};
pub use logging::init_logger;
