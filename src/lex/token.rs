use super::TokenType;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token<'a, 'b> {
    ty: TokenType,
    content: &'a str,
    source_filename: Option<&'b str>,
    index: usize,
    line: usize,
    linespan: (usize, usize),
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct TokenBuilder<'a, 'b> {
    ty: Option<TokenType>,
    content: Option<&'a str>,
    source_filename: Option<Option<&'b str>>,
    index: Option<usize>,
    line: Option<usize>,
    linespan: Option<(usize, usize)>
}

impl <'b> Token<'static, 'b> {
    pub fn new_eof(source_filename: Option<&'b str>) -> Self {
        Token {
            ty: TokenType::EOF,
            content: "EOF",
            source_filename: source_filename,
            index: 0,
            line: 0,
            linespan: (0, 3)
        }
    }
}

impl <'a, 'b> Token<'a, 'b> {
    pub fn get_type(&self) -> TokenType {
        self.ty
    }

    pub fn get_content(&self) -> &'a str {
        self.content
    }

    pub fn get_graphviz_name(&self) -> String {
        format!("{:?}_{}_{}", self.ty, self.source_filename.unwrap_or(""), self.index)
    }
}

impl <'a, 'b> TokenBuilder<'a, 'b> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(&self) -> Option<Token<'a, 'b>> {
        macro_rules! extract {
            ($id:expr) => {
                if let Some(val) = $id { val }
                else { return None; }
            }
        }
        Some(Token {
            ty: extract!(self.ty),
            content: extract!(self.content),
            source_filename: extract!(self.source_filename),
            index: extract!(self.index),
            line: extract!(self.line),
            linespan: extract!(self.linespan)
        })
    }

    pub fn with_type(mut self, ty: TokenType) -> Self {
        self.ty = Some(ty);
        self
    }

    pub fn with_content(mut self, content: &'a str) -> Self {
        self.content = Some(content);
        self
    }

    pub fn with_source_filename(mut self, source_filename: Option<&'b str>) -> Self {
        self.source_filename = Some(source_filename);
        self
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    pub fn with_linespan(mut self, from: usize, to: usize) -> Self {
        self.linespan = Some((from, to));
        self
    }
}
