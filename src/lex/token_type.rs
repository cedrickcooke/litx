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
    EOF
}
