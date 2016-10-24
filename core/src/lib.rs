#![crate_type = "lib"]
#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features,
        unused_import_braces, unused_qualifications)]
#![warn(missing_docs)]

//! This is a base library for dealing with DICOM information and communication.
//!
//! Sorry, no example yet!
//!

#[macro_use]
extern crate lazy_static;
extern crate byteorder;
extern crate encoding;
#[macro_use]
extern crate quick_error;


pub mod attribute;
pub mod error;
pub mod parser;
pub mod transfer_syntax;
pub mod data_element;
pub mod meta;

mod util;
