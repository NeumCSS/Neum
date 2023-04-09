#![crate_type = "lib"]

//! ![Neum](https://github.com/AMTitan/Neum/raw/master/assets/logo.svg)
//! # Example
//! ```
//! use neum_parse::*;
//! let file = "example.neum".to_string(); // Used just for giving errors
//! let content = std::fs::read_to_string(file.clone()).expect("Cant read file");
//! println!("Tokens: {:?}", parse::parse(lexer::lex(file.clone(), content.clone()), file, content));
//! ```

#[doc(hidden)]
pub mod error;
#[doc(hidden)]
pub mod lexer;
#[doc(hidden)]
pub mod parse;

pub struct Neum {
    file: Option<String>,
    content: String,
    converts: Vec<(parse::Name, Vec<lexer::Token>)>,
}

impl Neum {
    pub fn new(content: String, file: Option<String>) -> Result<Neum, error::NeumError> {
        let converts = parse::parse(
            lexer::lex(file.clone(), content.clone())?,
            file.clone(),
            content.clone(),
        )?;
        Ok(Neum {
            file,
            content,
            converts,
        })
    }

    pub fn convert(&self, input: String) -> Option<String> {
        parse::converts(self.converts.clone(), input)
    }

    pub fn add(&mut self, content: String, file: Option<String>) -> Result<(), error::NeumError> {
        let neum = Neum::new(content, file)?;
        for i in neum.converts {
            self.converts.push(i);
        }
        Ok(())
    }
}
