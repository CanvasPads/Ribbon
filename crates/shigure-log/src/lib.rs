/// A location information for  nodes
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Loc {
    pub start: usize,
    pub end: usize,
}

pub enum MessageLevel {
    Info,
    Warning,
    Error,
}

pub struct Hint {
    pub level: MessageLevel,
    pub loc: Loc,
    pub message: String,
}

pub struct Message {
    pub level: MessageLevel,
    pub pos: usize,
    pub title: String,
    pub hints: Vec<Hint>,
}

pub struct Logger<'a> {
    path: &'a String,
    input: &'a String,
    index: Vec<usize>,
    len: usize,
}

impl<'a> Logger<'a> {
    pub fn new(path: &'a String, input: &'a String) -> Self {
        let index = Self::index_input(input, input.len());
        Logger {
            path,
            input,
            index,
            len: input.len(),
        }
    }

    fn get_char(input: &'a String, len: usize, idx: usize) -> Option<char> {
        if idx < len {
            Some(input.as_bytes()[idx] as char)
        } else {
            None
        }
    }

    fn get_current_lines(&self, start: usize, end: usize) -> String {
        let mut line_start = start;
        while line_start > 0 {
            line_start -= 1;
            if let Some('\n') = Self::get_char(self.input, self.len, line_start) {
                line_start += 1;
                break;
            }
        }
        let mut line_end = end;
        while line_end < self.len {
            line_end += 1;
            if let Some('\n') = Self::get_char(self.input, self.len, line_end) {
                // line_end += 1; // By commentting out, ignore last \n
                break;
            }
        }

        let bytes = &self.input.as_bytes()[line_start..line_end];
        String::from_utf8(bytes.to_vec()).expect("Failed to create string")
    }

    fn pos_to_line(&self, pos: usize) -> usize {
        let mut line = 1;
        for i in 0..self.index.len() - 1 {
            if self.index[i] <= pos && self.index[i + 1] >= pos {
                return line;
            }
            line += 1;
        }
        line
    }

    fn index_input(input: &'a String, len: usize) -> Vec<usize> {
        let mut lines: Vec<usize> = vec![0];
        let mut idx = 1;
        while idx < len {
            if let Some('\n') = Self::get_char(input, len, idx) {
                lines.push(idx);
            }
            idx += 1;
        }
        lines
    }

    pub fn issue(&self, message: Message) {
        match message.level {
            MessageLevel::Info => print!("\x1b[1;34minfo"),
            MessageLevel::Warning => print!("\x1b[1;33mwarning"),
            MessageLevel::Error => print!("\x1b[1;31merror"),
        }
        let ypos = self.pos_to_line(message.pos);
        let xpos = message.pos - self.index[ypos - 1] + 1;
        print!("\x1b[0m: ");
        println!("{}", message.title);
        print!("-->  ");
        println!("{}:{}:{}", self.path, ypos, xpos);
        print!("{} | ", ypos);
        println!("{}", self.get_current_lines(message.pos, message.pos));
        for _i in 0..ypos.to_string().len() {
            print!(" ")
        }
        print!("   ");
        // ypos = \n
        if xpos > ypos {
            for _i in 0..xpos - ypos {
                print!(" ");
            }
        }

        println!("^");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let path = "/home/user/example/src/a.shi".to_string();
        let input = "let a = 1\nlet main = println(\"main\")\nconst b = 1".to_string();
        let logger = Logger::new(&path, &input);
        logger.issue(Message {
            level: MessageLevel::Info,
            pos: 21,
            title: "Unresolved symbol".into(),
            hints: vec![],
        });
        logger.issue(Message {
            level: MessageLevel::Warning,
            pos: 0,
            title: "Unresolved symbol".into(),
            hints: vec![],
        });
        logger.issue(Message {
            level: MessageLevel::Error,
            pos: 48,
            title: "Unresolved symbol".into(),
            hints: vec![],
        });
    }
}
