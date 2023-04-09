#[macro_export]
macro_rules! error {
    ( $( $x:expr ),* ) => {
        {
            eprintln!("Error: {}",format!($($x,)*));
            std::process::exit(1);
        }
    };
}

#[macro_export]
macro_rules! file_error {
    ( $file:expr, $content:expr, $location:expr, $error:expr ) => {{
        let (x, y) = $crate::error::get_loc($content.clone(), $location.start)
            .unwrap_or_else(|| $crate::error!("{} {}", $error, $file));
        let line = $crate::error::get_line($content, y - 1)
            .unwrap_or_else(|| $crate::error!("{} {}", $error, $file));
        eprintln!("Error: {} {}:{}:{}", $error, $file, y, x);
        eprintln!("{line}");
        eprintln!("{}{}", " ".repeat(x), "^".repeat($location.len()));
        std::process::exit(1);
    }};
}

pub fn get_loc(content: String, location: usize) -> Option<(usize, usize)> {
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

pub fn get_line(content: String, line: usize) -> Option<String> {
    Some(content.lines().nth(line)?.to_string())
}
