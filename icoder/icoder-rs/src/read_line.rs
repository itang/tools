///Read line
pub trait ReadLine {
    /// or read line
    fn or_read_line(self) -> String;
}

impl ReadLine for Option<String> {
    fn or_read_line(self) -> String {
        self.unwrap_or_else(read_line)
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("read line");
    buf.trim().to_string()
}
