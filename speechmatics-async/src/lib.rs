//! This is a crate to help you easily connect to the Speechmatics API (on-prem and SaaS) using the best programming language in the world!

#![warn(missing_docs)]

#[macro_use]
extern crate serde;

#[cfg(feature = "realtime")]
pub mod realtime;
#[cfg(feature = "batch")]
pub mod batch;