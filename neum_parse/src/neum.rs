use std::string::String;

/// This is a Neum converter object
pub struct Neum {
    #[doc(hidden)]
    pub converts: Vec<(parse::Name, Vec<lexer::Token>)>,
}

impl Neum {
    /// Creates a new Neum converter object
    /// ```
    /// # use neum_parse::*;
    /// let neum = Neum::new(".w-{} => width: {}px", None).unwrap(); // the file is just for error handling
    /// ```
    pub fn new<'a>(content: &'a str, file: Option<&'a str>) -> Result<Neum, error::NeumError<'a>> {
        let converts = parse::parse(lexer::lex(file, content)?, file, content)?;
        Ok(Neum { converts })
    }

    /// Takes your current Neum object and finds your input and gives the output
    /// ```
    /// # use neum_parse::*;
    /// let neum = Neum::new(".w-{} => width: {}px", None).unwrap();
    /// assert_eq!(neum.convert(".w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert(".mw-5"), None);
    /// ```
    /// This will also match the first item it gets
    /// ```
    /// # use neum_parse::*;
    /// let mut neum = Neum::new(".w-{}% => width: {}%", None).unwrap();
    /// neum.add(".w-{} => width: {}px", None).unwrap();
    /// assert_eq!(neum.convert(".w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert(".w-5%"), Some(String::from("width:5%;")));
    ///
    /// let mut neum = Neum::new(".w-{} => width: {}px", None).unwrap();
    /// neum.add(".w-{}% => width: {}%", None).unwrap();
    /// assert_eq!(neum.convert(".w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert(".w-5%"), Some(String::from("width:5%px;")));
    /// ```
    pub fn convert(&self, input: &str) -> Option<String> {
        parse::converts(self.converts.clone(), input)
    }

    /// Add some more Neum definitions to your Neum object, this will also add your item to the lowest priority
    /// ```
    /// # use neum_parse::*;
    /// let mut neum = Neum::new(".w-{} => width: {}px", None).unwrap();
    /// assert_eq!(neum.convert(".w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert(".mw-5"), None);
    ///
    /// neum.add(".mw-{} => max-width: {}px", None).unwrap();
    /// assert_eq!(neum.convert(".w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert(".mw-5"), Some(String::from("max-width:5px;")));
    /// ```
    pub fn add<'a>(
        &mut self,
        content: &'a str,
        file: Option<&'a str>,
    ) -> Result<(), error::NeumError<'a>> {
        let mut neum = Neum::new(content, file)?;
        self.converts.append(&mut neum.converts);
        Ok(())
    }
    
    /// Add some more Neum definitions to your Neum object, this will also add your item to the heighest priority
    /// ```
    /// # use neum_parse::*;
    /// let mut neum = Neum::new(".w-{} => width: {}px", None).unwrap();
    /// neum.add(".w-{}% => width: {}%", None).unwrap();
    /// assert_eq!(neum.convert(".w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert(".w-5%"), Some(String::from("width:5%px;")));
    ///
    /// let mut neum = Neum::new(".w-{} => width: {}px", None).unwrap();
    /// neum.add_priority(".w-{}% => width: {}%", None).unwrap();
    /// assert_eq!(neum.convert(".w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert(".w-5%"), Some(String::from("width:5%;")));
    /// ```
    pub fn add_priority<'a>(
        &mut self,
        content: &'a str,
        file: Option<&'a str>,
    ) -> Result<(), error::NeumError<'a>> {
        let mut neum = Neum::new(content, file)?;
        neum.converts.append(&mut self.converts);
        self.converts = neum.converts;
        Ok(())
    }
}
