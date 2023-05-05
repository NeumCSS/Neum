use neum::Neum;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub struct Builder {
    neum: Neum,
    files: Vec<PathBuf>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            neum: Neum::default(),
            files: Vec::new(),
        }
    }

    pub fn add(&mut self, path: PathBuf) -> anyhow::Result<(), neum::error::NeumError> {
        self.neum.add_priority(
            std::fs::read_to_string(path.clone()).unwrap(),
            Some(path.display().to_string()),
        )?;
        self.files.push(path);
        Ok(())
    }

    pub fn build(self) -> anyhow::Result<()> {
        for i in self.files {
            let mut reader =
                crate::doc::reader::Reader::new(fs::read_to_string(i.clone()).unwrap());

            let mut output = doc!().output.clone();
            output.push(
                i.as_path()
                    .strip_prefix(doc!().neum_folder.clone().unwrap_or(".".into()))?,
            );

            fs::create_dir_all(output.parent().unwrap())?;

            let mut file = fs::File::create(output.clone())?;

            while let Some(x) = reader.get_next() {
                let mut split = x.split(' ').collect::<Vec<&str>>();
                let mut markdown = true;
                let tag = match *split.first().unwrap() {
                    "///" => {
                        split.remove(0);
                        "h2"
                    }
                    "//" => {
                        split.remove(0);
                        "p"
                    }
                    "/*" => {
                        split.remove(0);
                        split.pop();
                        "p"
                    }
                    _ => {
                        markdown = false;
                        "pre"
                    }
                };
                file.write_all(
                    format!(
                        "<{tag}>{}</{tag}>\n",
                        match markdown {
                            true => md_to_html(&split.join(" ")),
                            false => split.join(" "),
                        }
                    )
                    .as_bytes(),
                )
                .unwrap();
            }

            println!("{output:?}");
        }
        Ok(())
    }
}

use emojicons::EmojiFormatter;
use pulldown_cmark::{html, Options, Parser};

pub fn md_to_html(input: &str) -> String {
    let input = EmojiFormatter(input).to_string();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
