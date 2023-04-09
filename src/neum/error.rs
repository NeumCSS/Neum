use std::ops::Range;

#[macro_export]
macro_rules! error {
    ( $( $x:expr ),* ) => {
        {
            eprintln!("Error: {}",format!($($x,)*));
            std::process::exit(1);
        }
    };
}

pub fn file_error(file: String, content: String, location: Range<usize>, error: &str) {
    let (x, y) =
        get_loc(content.clone(), location.start).unwrap_or_else(|| error!("{error} {file}"));
    let line = get_line(content, y - 1).unwrap_or_else(|| error!("{error} {file}"));
    eprintln!("Error: {error} {file}:{y}:{x}");
    eprintln!("{line}");
    eprintln!("{}{}", " ".repeat(x), "^".repeat(location.len()));
    std::process::exit(1);
}

fn get_loc(content: String, location: usize) -> Option<(usize, usize)> {
    let mut y = 0;
    let mut current = 0;
    for line in content.split('\n') {
        y += 1;
        let old = current;
        current += 1 + line.len();
        if old < location && current > location {
            return Some((location - old, y));
        }
    }
    None
}

fn get_line(content: String, line: usize) -> Option<String> {
    Some(content.lines().nth(line)?.to_string())
}
