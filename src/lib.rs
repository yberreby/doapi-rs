#![feature(custom_derive, plugin)]
#![feature(const_fn)]
#![plugin(serde_macros)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate quick_error;

extern crate serde;
extern crate serde_json;
extern crate hyper;

pub mod common;

mod result;
pub use self::result::DoResult;

pub mod client;
pub use self::client::Client;

pub mod request;
pub mod response;
