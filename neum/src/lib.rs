#![crate_type = "lib"]

//! ![Neum](https://github.com/AMTitan/Neum/raw/master/assets/logo.svg)
//! # Example
//! ```no_run
//! use neum_parse::*;
//! let file = "width.neum"; // Used just for giving errors
//! let content = std::fs::read_to_string(file.clone()).expect("Cant read file");
//! let neum = Neum::new(&content, Some(&file.to_string())).unwrap();
//! assert_eq!(neum.convert(".w-50%"), Some(String::from("width:50%;")));
//! ```

pub use neum_parse::*;

include!("neum.rs");

include!(concat!(env!("OUT_DIR"), "/formated.rs"));
