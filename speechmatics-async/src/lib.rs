#[macro_use]
extern crate serde;

#[cfg(feature = "realtime")]
pub mod realtime;
#[cfg(feature = "batch")]
pub mod batch;