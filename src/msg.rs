use std::fmt;

use colored::{ Colorize, ColoredString };

#[derive(RustcDecodable, Clone, Copy, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Level {
    warning,
    error,
    note,
    help,
}

impl Level {
    pub fn colorize(&self, s: &str) -> ColoredString {
        match *self {
            Level::warning => s.yellow(),
            Level::error => s.red(),
            Level::note => s.green(),
            Level::help => s.blue(),
        }.bold()
    }

    pub fn to_string(&self) -> &'static str {
        match *self {
            Level::warning => "warning",
            Level::error => "error",
            Level::note => "note",
            Level::help => "help",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, mut f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.colorize(self.to_string()))
    }
}

#[derive(RustcDecodable)]
pub struct Text {
    pub highlight_start: Option<usize>,
    pub highlight_end: Option<usize>,
    pub text: String,
}

impl Text {
    fn fmt(&self, mut f: &mut fmt::Formatter, level: Level) -> fmt::Result {
        if let (Some(start), Some(end)) = (self.highlight_start, self.highlight_end) {
            write!(f, "{}{}{}",
                &self.text[..(start - 1)],
                level.colorize(&self.text[(start - 1)..(end - 1)]),
                &self.text[(end - 1)..])?;
        } else {
            write!(f, "{}", self.text)?;
        }
        Ok(())
    }
}

#[derive(RustcDecodable)]
pub struct Expansion {
    pub span: Span
}

#[derive(RustcDecodable)]
pub struct Span {
    pub column_start: usize,
    pub file_name: String,
    pub line_start: usize,
    pub text: Vec<Text>,
    pub expansion: Option<Box<Expansion>>
}

impl Span {
    pub fn fmt(&self, mut f: &mut fmt::Formatter, level: Level) -> fmt::Result {
        if let Some(ref expansion) = self.expansion {
            expansion.span.fmt(f, level)?;
        } else {
            write!(f, " {} {}:{}:{}\n",
                "    -->".bold().blue(),
                self.file_name,
                self.line_start,
                self.column_start)?;
            write!(f, "      {}\n", "|".bold().blue())?;
            if let Some(text) = self.text.first() {
                write!(f, " {:>4} {} ",
                    self.line_start.to_string().bold().blue(),
                    "|".bold().blue())?;
                text.fmt(f, level)?;
                write!(f, "\n")?;
            }
            write!(f, "      {}\n", "|".bold().blue())?;
        }
        Ok(())
    }
}

#[derive(RustcDecodable)]
pub struct Message {
    pub level: Level,
    pub message: String,
    pub spans: Vec<Span>,
    pub children: Vec<Message>,
}

impl Message {
    fn external_macro() -> Message {
        Message {
            level: Level::note,
            message: "this error originates in a macro outside of the current crate".to_owned(),
            spans: vec![],
            children: vec![],
        }
    }
}

impl Message {
    fn fmt(&self, mut f: &mut fmt::Formatter, child: bool) -> fmt::Result {
        if child && self.spans.is_empty() {
            write!(f, "      {} {}: {}\n",
                "=".bold().blue(),
                self.level.to_string().bold(),
                self.message)?;
        } else {
            write!(f, "{}{}{}\n",
                self.level,
                ": ".bold(),
                self.message.bold())?;
            for span in &self.spans {
                span.fmt(f, self.level)?;
            }
            for child in &self.children {
                child.fmt(f, true)?;
            }
            if self.spans.iter().any(|span| span.expansion.is_some()) {
                Message::external_macro().fmt(f, true)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(RustcDecodable)]
pub struct Line {
    pub message: Message,
}

impl fmt::Display for Line {
    fn fmt(&self, mut f: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(f, false)
    }
}
