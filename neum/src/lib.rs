#![crate_type = "lib"]

//! ![Neum](https://github.com/AMTitan/Neum/raw/master/assets/logo.svg)
//! # Example
//! ```no_run
//! use neum::*;
//! let file = "width.neum"; // Used just for giving errors
//! let content = std::fs::read_to_string(file.clone()).expect("Cant read file");
//! let mut neum = Neum::new(&content, Some(&file.to_string())).unwrap();
//! assert_eq!(neum.convert("w-50%"), Some(String::from("width:50%;")));
//! ```
//!
//! ```no_run
//! use neum::*;
//! let mut neum = Neum::default();
//! assert_eq!(neum.convert("w-50%"), Some(String::from("width:50%;")));
//! ```
//!
//! If you want to see the default classes that are provided, go to [defaults](defaults/index.html), if you want to see how to use the library go to [Neum](struct.Neum.html)

pub use neum_parse::*;

include!("neum.rs");

include!(concat!(env!("OUT_DIR"), "/formated.rs"));

pub mod defaults;
