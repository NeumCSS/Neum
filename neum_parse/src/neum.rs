use std::sync::Arc;

/// This is a Neum converter object
#[derive(Debug, Clone)]
pub struct Neum {
    #[doc(hidden)]
    pub converts: Arc<Vec<(parse::Name, Vec<lexer::Token>)>>,

    #[doc(hidden)]
    pub consts: Arc<hashbrown::HashMap<std::string::String, Vec<lexer::Token>>>,

    #[doc(hidden)]
    pub cache: Arc<hashbrown::HashMap<std::string::String, Option<std::string::String>>>,
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
        Ok(Neum { converts: Arc::new(output.dynamics.to_vec()), consts: Arc::new(output.statics), cache: Arc::new(hashbrown::HashMap::new()) })
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
        self.cache = Arc::new(hashbrown::HashMap::new());
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
        parse::converts(self.converts.clone(), self.consts.clone(), &mut Arc::make_mut(&mut self.cache), input.as_ref())
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
        Arc::get_mut(&mut self.converts).unwrap().append(Arc::get_mut(&mut neum.converts).unwrap());
        Arc::get_mut(&mut self.consts).unwrap().extend((*neum.consts).clone());
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
        Arc::get_mut(&mut neum.converts).unwrap().append(Arc::get_mut(&mut self.converts).unwrap());
        self.converts = neum.converts;
        Arc::get_mut(&mut neum.consts).unwrap().extend((*self.consts).clone());
        self.consts = neum.consts;
        Ok(())
    }

    /// Returns a empty Neum type with nothing defined
    #[inline(always)]
    pub fn empty() -> Neum {
        Neum {
            converts: Arc::new(Vec::new()),
            consts: Arc::new(hashbrown::HashMap::new()),
            cache: Arc::new(hashbrown::HashMap::new())
        }
    }

    /// Combine two Neum items, the first item has priority over the others
    /// ```
    /// # use neum_parse::*;
    /// let file_one = Neum::new("color => red\nhello-{} => hello {}", None).unwrap();
    ///
    /// let file_two = Neum::new("color => yellow\nhello-{} => goodbye {}", None).unwrap();
    ///
    /// // Note that file_two is going to have more priority to file_one
    /// let mut neum = Neum::empty();
    /// neum.combine(file_one);
    /// neum.combine(file_two);
    ///
    /// assert_eq!(neum.convert("color"), Some(String::from("red;")));
    /// assert_eq!(neum.convert("hello-world"), Some(String::from("hello world;")));
    /// ```
    #[inline(always)]
    pub fn combine(
        &mut self,
        neum: &mut Neum,
    ) {
        Arc::get_mut(&mut self.converts).unwrap().append(Arc::get_mut(&mut neum.converts).unwrap());
        Arc::get_mut(&mut neum.consts).unwrap().extend(Arc::get_mut(&mut self.consts).unwrap().clone());
        self.consts = neum.consts.clone();
    }

    /// Combine two Neum items, the first item has priority over the others
    /// ```
    /// # use neum_parse::*;
    /// let file_one = Neum::new("color => red\nhello-{} => hello {}", None).unwrap();
    ///
    /// let file_two = Neum::new("color => yellow\nhello-{} => goodbye {}", None).unwrap();
    ///
    /// // Note that file_two is going to have more priority to file_one
    /// let mut neum = Neum::empty();
    /// neum.combine(file_one);
    /// neum.combine_priority(file_two);
    ///
    /// assert_eq!(neum.convert("color"), Some(String::from("yellow;")));
    /// assert_eq!(neum.convert("hello-world"), Some(String::from("goodbye world;")));
    /// ```
    #[inline(always)]
    pub fn combine_priority(
        &mut self,
        neum: &mut Neum,
    ) {
        Arc::get_mut(&mut neum.converts).unwrap().append(Arc::get_mut(&mut self.converts).unwrap());
        Arc::get_mut(&mut self.consts).unwrap().extend(Arc::get_mut(&mut neum.consts).unwrap().clone());
        self.converts = neum.converts.clone();
    }
}
