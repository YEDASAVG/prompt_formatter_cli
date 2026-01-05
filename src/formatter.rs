use regex::Regex;

pub enum FormatMode {
    Compact,
    Readable,
    Minimal,
}
pub struct Formatter {
    mode: FormatMode
}

impl Formatter {
    pub fn new(mode: FormatMode) -> Self {
        Formatter { mode: mode }
    }

    pub fn format(&self, text: &str) -> String {
        let re = Regex::new(r" {2,}").unwrap();
        re.replace_all(text, " ").to_string();

        let text = self.trim_lines(&text);
        let text = self.normalize_newlines(&text);
        text
    }

    pub fn trim_lines(&self, text: &str) -> String { // helper function 
        text.lines().map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
    }
    pub fn normalize_newlines(&self, text: &str) -> String { // helper functions
        let re = Regex::new(r"\n{3,}").unwrap();
        re.replace_all(text, "\n\n").to_string()
    }
}