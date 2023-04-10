use std::string::String;

/// This is a Neum converter object
#[derive(Debug)]
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

    /// Returns a empty Neum type with nothing defined
    pub fn empty() -> Neum {
        Neum {
            converts: Vec::new()
        }
    }

    /// Combine two Neum items, the first item has priority over the others
    /// ```no_run
    /// # use neum_parse::*;
    /// # use std::fs;
    /// let file = "width.neum";
    /// let file_one = Neum::new(&fs::read_to_string(file).unwrap(), Some(file)).unwrap();
    ///
    /// let file = "height.neum";
    /// let file_two = Neum::new(&fs::read_to_string(file).unwrap(), Some(file)).unwrap();
    ///
    /// // Note that file_two is going to have less priority to file_one
    /// let neum = Neum::empty().combine(file_one).combine(file_two);
    ///
    /// assert_eq!(neum.convert(".w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert(".h-5"), Some(String::from("height:5px;")));
    /// ```
    pub fn combine(
        self,
        neum: Neum,
    ) -> Neum {
        let mut neum_clone = neum.converts;
        let mut self_clone = self.converts;
        self_clone.append(&mut neum_clone);
        Neum{converts:self_clone}
    }

    /// Combine two Neum items, the first item has priority over the others
    /// ```no_run
    /// # use neum_parse::*;
    /// # use std::fs;
    /// let file = "width.neum";
    /// let file_one = Neum::new(&fs::read_to_string(file).unwrap(), Some(file)).unwrap();
    ///
    /// let file = "height.neum";
    /// let file_two = Neum::new(&fs::read_to_string(file).unwrap(), Some(file)).unwrap();
    ///
    /// // Note that file_two is going to have more priority to file_one
    /// let neum = Neum::empty().combine(file_one).combine_priority(file_two);
    ///
    /// assert_eq!(neum.convert(".w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert(".h-5"), Some(String::from("height:5px;")));
    /// ```
    pub fn combine_priority(
        self,
        neum: Neum,
    ) -> Neum {
        let mut neum_clone = neum.converts;
        let mut self_clone = self.converts;
        neum_clone.append(&mut self_clone);
        Neum{converts:neum_clone}
    }
}
