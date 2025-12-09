#[macro_export]
macro_rules! log {
    ( $($e:expr),* ) => {{
        use std::io::Write;
        let mut f =
            std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("output/logs")
                .unwrap();
        writeln!(f, $($e),*).unwrap();
        f.flush().unwrap();
    }};
}

#[macro_export]
macro_rules! log_lexer {
    ( $file:expr, $($e:expr),* ) => {{
        use std::io::Write;
        let mut f =
            std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open($file)
                .unwrap();
        writeln!(f, $($e),*).unwrap();
        f.flush().unwrap();
    }};
}
