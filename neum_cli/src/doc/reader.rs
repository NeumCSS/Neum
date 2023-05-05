pub struct Reader {
    input: String
}

impl Reader {
    pub fn new(input: String) -> Reader {
        Reader {
            input
        }
    }

    pub fn get_next(&mut self) -> Option<String> {
        let mut lines = self.input.lines();
        let mut returns = String::new();
        let mut first = true;
        let mut count = 0;
        while count != 0 || first {
            let next = lines.next()?;
            if !next.is_empty() {
                if returns.starts_with("/*") {
                    count+=next.chars().collect::<Vec<_>>().windows(2).map(|x| match x {
                        ['/', '*'] => 1,
                        ['*', '/'] => -1,
                        _ => 0
                    }).sum::<i32>();
                }
                else {
                    count+=next.chars().map(|x| match x {
                        '{' => 1,
                        '}' => -1,
                        _ => 0
                    }).sum::<i32>();
                }
                first = false;
                returns.push_str(&format!("\n{next}"));
            }
        }
        self.input = lines.collect::<Vec<&str>>().join("\n");
        Some(returns.trim().to_string())
    }
}
