use std::{
    fmt,
    fs::File,
    io::Write as _,
    path::Path,
    sync::{Mutex, OnceLock},
};

pub const LOG_FILE_PATH: &str = "logs";
static LOG_FILE: OnceLock<Mutex<File>> = OnceLock::new();

#[doc(hidden)]
pub fn _log_ex(
    output_file: impl AsRef<Path>,
    ctx_file: &'static str,
    ctx_line: u32,
    args: fmt::Arguments,
) {
    // Initialize the file once, or get the existing handle
    let file_lock = LOG_FILE.get_or_init(|| {
        println!("DEBUG: Initializing log file at {:?}", output_file.as_ref());
        let f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(output_file)
            .expect("Failed to open log file");
        Mutex::new(f)
    });

    if let Ok(mut file) = file_lock.lock() {
        let _ = writeln!(file, "[{ctx_file}:{ctx_line}] {}", args);
    }
}

#[macro_export]
macro_rules! log {
    ($($args:tt)*) => {
        $crate::errors::logging::_log_ex($crate::errors::logging::LOG_FILE_PATH, file!(), line!(), format_args!($($args)*));
    };
}

#[macro_export]
macro_rules! log_lexer {
    ( $file:expr, $($args:tt)* ) => {{
        $crate::errors::logging::_log_ex($crate::errors::logging::LOG_FILE_PATH, file!(), line!(), format_args!($($args)*))
    }};
}
