use std::{iter::Peekable, str::Bytes};

pub struct Scanner<'a> {
    text: Peekable<Bytes<'a>>,
    previous: Option<u8>,
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text: text.bytes().peekable(),
            previous: None,
        }
    }
}

fn get_hex(text: &mut Peekable<Bytes<'_>>) -> Option<u8> {
    let mut num = String::new();
    if let Some(next) = text.next() {
        num.push(next as char);
    } else {
        panic!("\\x expects value")
    }
    if let Some(next) = text.next() {
        num.push(next as char);
    } else {
        panic!("\\x expects 2 values")
    }

    if let Ok(num) = u8::from_str_radix(num.as_str(), 16) {
        Some(num)
    } else {
        None
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if matches!(self.text.peek(), None) && matches!(self.previous, None) {
            return None;
        }
        let mut buffer = vec![];

        if let Some(previous) = self.previous {
            self.previous = None;
            match previous {
                b',' | b':' | b'.' | b';' | b'\n' => return Some(vec![previous]),
                x => unreachable!("{x:?} should not be in this place"),
            }
        }

        let mut in_str = false;

        while let Some(ch) = self.text.next() {
            if !in_str {
                match ch {
                    b' ' | b'\t' => {
                        if !buffer.is_empty() {
                            return Some(buffer);
                        }
                    }
                    b',' | b':' | b'.' | b';' | b'\n' => {
                        if buffer.is_empty() {
                            return Some(vec![ch]);
                        } else {
                            self.previous = Some(ch);
                            return Some(buffer);
                        }
                    }
                    b'\r' if matches!(self.text.peek(), Some(b'\n')) => {
                        if buffer.is_empty() {
                            return Some(vec![b'\n']);
                        } else {
                            self.previous = self.text.next();
                            return Some(buffer);
                        }
                    }
                    b'"' if buffer.is_empty() => {
                        buffer.push(ch);
                        in_str = true;
                    }
                    b'"' => panic!("unexpected '\"'"),

                    ch => {
                        buffer.push(ch);
                    }
                }
            } else if ch == b'"' {
                buffer.push(ch);
                return Some(buffer);
            } else if ch == b'\\' {
                if let Some(next) = self.text.next() {
                    match next {
                        b'n' => buffer.push(b'\n'),
                        b't' => buffer.push(b'\t'),
                        b'r' => buffer.push(b'\r'),
                        b'x' => buffer
                            .push(get_hex(&mut self.text).expect("couldnt convert hex literal")),
                        x => buffer.push(x),
                    }
                } else {
                    panic!("escaping nothing");
                }
            } else {
                buffer.push(ch);
            }
        }

        if in_str {
            panic!("unclosed string");
        }

        Some(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::Scanner;

    macro_rules! str_vec {
        ($string:expr) => {{
            let string: String = $string.to_owned();
            string.bytes().collect::<Vec<u8>>()
        }};
    }

    #[test]
    fn test_next() {
        let text = "hello this is test";
        let mut scanner = Scanner::new(text);
        assert_eq!(scanner.next(), Some(str_vec!("hello")));
        assert_eq!(scanner.next(), Some(str_vec!("this")));
        assert_eq!(scanner.next(), Some(str_vec!("is")));
        assert_eq!(scanner.next(), Some(str_vec!("test")));
        assert_eq!(scanner.next(), None);
    }
    #[test]
    fn test_commas() {
        let text = "hello, this is : test";
        let mut scanner = Scanner::new(text);
        assert_eq!(scanner.next(), Some(str_vec!("hello")));
        assert_eq!(scanner.next(), Some(str_vec!(",")));
        assert_eq!(scanner.next(), Some(str_vec!("this")));
        assert_eq!(scanner.next(), Some(str_vec!("is")));
        assert_eq!(scanner.next(), Some(str_vec!(":")));
        assert_eq!(scanner.next(), Some(str_vec!("test")));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn test_whitespace() {
        let text = "   \t\thello   \n\t \n";
        let mut scanner = Scanner::new(text);
        assert_eq!(scanner.next(), Some(str_vec!("hello")));
        assert_eq!(scanner.next(), Some(str_vec!("\n")));
        assert_eq!(scanner.next(), Some(str_vec!("\n")));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn test_crlf() {
        let text = "XOR R0, R0, R0\r\n";
        let mut scanner = Scanner::new(text);
        assert_eq!(scanner.next(), Some(str_vec!("XOR")));
        assert_eq!(scanner.next(), Some(str_vec!("R0")));
        assert_eq!(scanner.next(), Some(str_vec!(",")));
        assert_eq!(scanner.next(), Some(str_vec!("R0")));
        assert_eq!(scanner.next(), Some(str_vec!(",")));
        assert_eq!(scanner.next(), Some(str_vec!("R0")));
        assert_eq!(scanner.next(), Some(str_vec!("\n")));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn test_escaped() {
        let text = "\"hello \\\" !!\"";
        let mut scanner = Scanner::new(text);
        assert_eq!(scanner.next(), Some(str_vec!("\"hello \" !!\"")));
        let text = "\"\\x69\"";
        let mut scanner = Scanner::new(text);
        assert_eq!(scanner.next(), Some(str_vec!("\"i\"")));
    }
}
