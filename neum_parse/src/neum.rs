use std::rc::Rc;

/// This is a Neum converter object
#[derive(Debug, Clone)]
pub struct Neum {
    #[doc(hidden)]
    pub converts: Vec<(parse::Name, Vec<lexer::Token>)>,

    #[doc(hidden)]
    pub consts: hashbrown::HashMap<std::string::String, Vec<lexer::Token>>,

    #[doc(hidden)]
    pub cache: hashbrown::HashMap<std::string::String, Option<std::string::String>>,
}

impl Neum {
    /// Creates a new Neum converter object
    /// ```
    /// # use neum_parse::*;
    /// let neum = Neum::new("w-{} => width: {}px", None).unwrap(); // the file is just for error handling
    /// ```
    pub fn new<S: AsRef<str> + std::fmt::Display>(content: S, file: Option<S>) -> Result<Neum, error::NeumError> {
        let file = file.map(|x| x.as_ref().to_string());
        let output = parse::parse(lexer::lex(file.clone(), content.as_ref().to_string())?, file, content.as_ref().to_string())?;
        Ok(Neum { converts: output.dynamics, consts: output.statics, cache: hashbrown::HashMap::new() })
    }

    /// Refresh the cache so that if a definition changed it will actually give a different responce
    /// ```
    /// # use neum_parse::*;
    /// let mut neum = Neum::new("w-{} => width: {}px", None).unwrap();
    /// assert_eq!(neum.convert("w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert("mw-5"), None);
    /// ```
    /// This will also match the first item it gets
    /// ```
    /// # use neum_parse::*;
    /// let mut neum = Neum::new("w-{}% => width: {}%", None).unwrap();
    /// assert_eq!(neum.convert("w-5%"), Some(String::from("width:5%;")));
    /// assert_eq!(neum.convert("w-5px"), None);
    ///
    /// neum.add("w-{}px => width: {}px", None).unwrap();
    ///
    /// assert_eq!(neum.convert("w-5%"), Some(String::from("width:5%;")));
    /// assert_eq!(neum.convert("w-5px"), None);
    ///
    /// neum.refresh();
    ///
    /// assert_eq!(neum.convert("w-5%"), Some(String::from("width:5%;")));
    /// assert_eq!(neum.convert("w-5px"), Some(String::from("width:5px;")));
    /// ```
    #[inline(always)]
    pub fn refresh(&mut self) {
        self.cache = hashbrown::HashMap::new();
    }

    /// Takes your current Neum object and finds your input and gives the output
    /// ```
    /// # use neum_parse::*;
    /// let mut neum = Neum::new("w-{} => width: {}px", None).unwrap();
    /// assert_eq!(neum.convert("w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert("mw-5"), None);
    /// ```
    /// This will also match the first item it gets
    /// ```
    /// # use neum_parse::*;
    /// let mut neum = Neum::new("w-{}% => width: {}%", None).unwrap();
    /// neum.add("w-{} => width: {}px", None).unwrap();
    /// assert_eq!(neum.convert("w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert("w-5%"), Some(String::from("width:5%;")));
    ///
    /// let mut neum = Neum::new("w-{} => width: {}px", None).unwrap();
    /// neum.add("w-{}% => width: {}%", None).unwrap();
    /// assert_eq!(neum.convert("w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert("w-5%"), Some(String::from("width:5%px;")));
    /// ```
    #[inline(always)]
    pub fn convert<S: AsRef<str>>(&mut self, input: S) -> Option<std::string::String> {
        parse::converts(self.converts.clone(), Rc::new(self.consts.clone()), &mut self.cache, input.as_ref())
    }

    /// Add some more Neum definitions to your Neum object, this will also add your item to the lowest priority
    /// ```
    /// # use neum_parse::*;
    /// let mut neum = Neum::new("w-{} => width: {}px", None).unwrap();
    /// assert_eq!(neum.convert("w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert("mw-5"), None);
    ///
    /// neum.add("mw-{} => max-width: {}px", None).unwrap();
    /// neum.refresh();
    ///
    /// assert_eq!(neum.convert("w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert("mw-5"), Some(String::from("max-width:5px;")));
    /// ```
    #[inline(always)]
    pub fn add<S: AsRef<str> + std::fmt::Display>(
        &mut self,
        content: S,
        file: Option<S>,
    ) -> Result<(), error::NeumError> {
        let mut neum = Neum::new(content, file)?;
        self.converts.append(&mut neum.converts);
        self.consts.extend(neum.consts);
        Ok(())
    }
    
    /// Add some more Neum definitions to your Neum object, this will also add your item to the heighest priority
    /// ```
    /// # use neum_parse::*;
    /// let mut neum = Neum::new("w-{} => width: {}px", None).unwrap();
    /// neum.add("w-{}% => width: {}%", None).unwrap();
    /// assert_eq!(neum.convert("w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert("w-5%"), Some(String::from("width:5%px;")));
    ///
    /// let mut neum = Neum::new("w-{} => width: {}px", None).unwrap();
    /// neum.add_priority("w-{}% => width: {}%", None).unwrap();
    /// assert_eq!(neum.convert("w-5"), Some(String::from("width:5px;")));
    /// assert_eq!(neum.convert("w-5%"), Some(String::from("width:5%;")));
    /// ```
    #[inline(always)]
    pub fn add_priority<S: AsRef<str> + std::fmt::Display>(
        &mut self,
        content: S,
        file: Option<S>,
    ) -> Result<(), error::NeumError> {
        let mut neum = Neum::new(content, file)?;
        neum.converts.append(&mut self.converts);
        self.converts = neum.converts;
        neum.consts.extend(self.consts.clone());
        self.consts = neum.consts;
        Ok(())
    }

    /// Returns a empty Neum type with nothing defined
    #[inline(always)]
    pub fn empty() -> Neum {
        Neum {
            converts: Vec::new(),
            consts: hashbrown::HashMap::new(),
            cache: hashbrown::HashMap::new()
        }
    }

    /// Combine two Neum items, the first item has priority over the others
    /// ```
    /// # use neum_parse::*;
    /// let file_one = Neum::new("color => red", None).unwrap();
    ///
    /// let file_two = Neum::new("color => yellow", None).unwrap();
    ///
    /// // Note that file_two is going to have more priority to file_one
    /// let mut neum = Neum::empty().combine(file_one).combine(file_two);
    ///
    /// assert_eq!(neum.convert("color"), Some(String::from("red;")));
    /// ```
    #[inline(always)]
    pub fn combine(
        self,
        neum: Neum,
    ) -> Neum {
        let mut neum_clone = neum.converts;
        let mut self_clone = self.converts;
        neum_clone.append(&mut self_clone);
        let mut neum_clone_consts = neum.consts;
        let self_clone_consts = self.consts;
        neum_clone_consts.extend(self_clone_consts);
        Neum{converts:neum_clone, consts: neum_clone_consts, cache: self.cache}
    }

    /// Combine two Neum items, the first item has priority over the others
    /// ```
    /// # use neum_parse::*;
    /// let file_one = Neum::new("color => red", None).unwrap();
    ///
    /// let file_two = Neum::new("color => yellow", None).unwrap();
    ///
    /// // Note that file_two is going to have more priority to file_one
    /// let mut neum = Neum::empty().combine(file_one).combine_priority(file_two);
    ///
    /// assert_eq!(neum.convert("color"), Some(String::from("yellow;")));
    /// ```
    #[inline(always)]
    pub fn combine_priority(
        self,
        neum: Neum,
    ) -> Neum {
        let mut neum_clone = neum.converts;
        let mut self_clone = self.converts;
        self_clone.append(&mut neum_clone);
        let neum_clone_consts = neum.consts;
        let mut self_clone_consts = self.consts;
        self_clone_consts.extend(neum_clone_consts);
        Neum{converts:self_clone, consts: self_clone_consts, cache: self.cache}
    }
}
