use regex::Regex;

macro_rules! decl_regex {
    ($id:ident, $s:expr) => (
        lazy_static! {
            static ref $id : Regex = Regex::new($s).unwrap();
        }
    )
}

decl_regex!(RGX_BLANK_LINE, r#"^(?m)\n\r?(\s*\n\r?)+"#);
decl_regex!(RGX_CLOSE_CMNT, r#"^!\}"#);
decl_regex!(RGX_CLOSE_EXPR, r#"^\}"#);
decl_regex!(RGX_CLOSE_MATH, r#"^\$\}"#);
decl_regex!(RGX_ESCAPED,     r#"^\\\S"#);
decl_regex!(RGX_KEY_START,  r#"^:"#);
decl_regex!(RGX_NEW_LINE,   r#"^(?m)\n\r?"#);
decl_regex!(RGX_NUMBER,     r#"^[0-9]+(\.[0-9]+)?"#);
decl_regex!(RGX_OPEN_CMNT,  r#"^\{!"#);
decl_regex!(RGX_OPEN_EXPR,  r#"^\{"#);
decl_regex!(RGX_OPEN_MATH,  r#"^\{\$"#);
decl_regex!(RGX_QUOTE,      r#"^""#);
decl_regex!(RGX_SPACE,      r#"^\s+"#);
// TODO: Double check the correctness of this.
// So far: whitespace, quote, colon, {, }, $}, !), \ are forbidden.
decl_regex!(RGX_WORD,       r#"^[^\s":\{\}(\$\})(!\})\\]+"#);
decl_regex!(RGX_CHAR,       r#"^\S"#);

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum TokenType {
    BlankLine,
    Char,
    CloseComment,
    CloseExpression,
    CloseMath,
    Escaped,
    KeyStart,
    NewLine,
    Number,
    OpenComment,
    OpenExpression,
    OpenMath,
    Quote,
    Space,
    Word,
}

lazy_static! {
    static ref REGEX_TOKENTYPE_PAIR: [(&'static Regex, TokenType); 15] = [
        // Opening an expression needs to be checked after opening comments/maths.
        // Otherwise, the order of the literals shouldn't really matter.
        // Here escaped characters are maximal priority though, to help guarantee their semantic meaning.
        (&RGX_ESCAPED, TokenType::Escaped),
        (&RGX_OPEN_CMNT, TokenType::OpenComment),
        (&RGX_OPEN_MATH, TokenType::OpenMath),
        (&RGX_OPEN_EXPR, TokenType::OpenExpression),
        (&RGX_CLOSE_CMNT, TokenType::CloseComment),
        (&RGX_CLOSE_MATH, TokenType::CloseMath),
        (&RGX_CLOSE_EXPR, TokenType::CloseExpression),
        (&RGX_KEY_START, TokenType::KeyStart),
        (&RGX_QUOTE, TokenType::Quote),
        // Blank-line checks must come before other whitespace.
        (&RGX_BLANK_LINE, TokenType::BlankLine),
        (&RGX_NEW_LINE, TokenType::NewLine),
        (&RGX_SPACE, TokenType::Space),
        // Check number before word: numbers are valid words by the regex.
        (&RGX_NUMBER, TokenType::Number),
        // Check word last in case I messed up the regex and theirs overlap with something.
        (&RGX_WORD, TokenType::Word),
        (&RGX_CHAR, TokenType::Char)
    ];
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Token<'a, 'b> {
    pub ty: TokenType,
    pub content: &'a str,
    source_file_name: Option<&'b str>,
    index: usize,
    line: usize,
    lspan: (usize, usize),
}

#[derive(Clone)]
pub struct Lexer<'a, 'b> {
    source_string: &'a str,
    source_file_name: Option<&'b str>,
    current_line: usize,
    current_line_index: usize,
    byte_index: usize
}

impl <'a> Lexer<'a, 'static> {
    pub fn new(source_string: &'a str) -> Lexer<'a, 'static> {
        Lexer {
            source_string: source_string,
            source_file_name: None,
            current_line: 0,
            current_line_index: 0,
            byte_index: 0
        }
    }
}

impl <'a, 'b> Lexer<'a, 'b> {
    pub fn new_with_filename(source_string: &'a str, source_file_name: &'b str) -> Lexer<'a, 'b> {
        Lexer {
            source_string: source_string,
            source_file_name: Some(source_file_name),
            current_line: 0,
            current_line_index: 0,
            byte_index: 0
        }
    }
}

impl <'a, 'b> Iterator for Lexer<'a, 'b> {
    type Item = Token<'a, 'b>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.byte_index == self.source_string.len() {
            return None;
        }
        let text = &self.source_string[self.byte_index..];
        for &(rgx, ty) in REGEX_TOKENTYPE_PAIR.iter() {
            if let Some(m) = rgx.find(text) {
                assert!(m.start() == 0);

                let token = Token {
                    ty: ty, content: m.as_str(),
                    source_file_name: self.source_file_name, index: self.byte_index, line: self.current_line,
                    lspan: (self.current_line_index, self.current_line_index + m.end())
                };

                self.byte_index += m.end();
                self.current_line_index += m.end();
                if ty == TokenType::NewLine || ty == TokenType::BlankLine {
                    self.current_line += m.as_str().chars().filter(|&c| c == '\n').count();
                    self.current_line_index = 0;
                }
                return Some(token);
            }
        }
        // TODO: I need to figure out some way to indicate an error here.
        unimplemented!();
    }
}
