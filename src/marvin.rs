use msg;

pub trait Marvinable {
    fn marvined(self) -> Self;
}

impl Marvinable for msg::Line {
    fn marvined(self) -> Self {
        msg::Line { message: self.message.marvined() }
    }
}

impl Marvinable for msg::Message {
    fn marvined(self) -> Self {
        msg::Message {
            level: self.level,
            message: self.message.marvined(),
            spans: self.spans,
            children: self.children.into_iter().map(Marvinable::marvined).collect(),
        }
    }
}

impl Marvinable for String {
    fn marvined(self) -> Self {
        if self.starts_with("use of moved value") {
            self + ". Again. There goes my hope you learned that by now. Serously. It's not that hard..."
        } else if self.starts_with("aborting due to previous error") {
            self + ". Maybe try a garbage-collected language?"
        } else {
            self
        }
    }
}
