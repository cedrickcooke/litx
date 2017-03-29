#![allow(dead_code)]

use ::lex::Token;
use ::lex::TokenType;
use std::iter::Peekable;

#[derive(Debug)]
pub enum ProductionType {
    S,
    Blocks,
    Block,
    Text,
    TextItem,
    Comment,
    CommentBody,
    CommentTerm,
    Expr,
    ExprBody,
    ExprItem,
    ExprProp,
    ExprIdent,
    ExprLiteral,
    Math,
    MathBody,
    MathTerm,
    String,
    StringBody,
    AnyWhiteSpace,
    SigWhiteSpace,
    WhiteSpace,
    Terminal
}

#[derive(Debug)]
pub enum Branch<'a, 'b> {
    Terminal(Token<'a, 'b>),
    Nonterminal(Production<'a, 'b>)
}

#[derive(Debug)]
pub struct Production<'a, 'b> {
    ty: ProductionType,
    children: Vec<Branch<'a, 'b>>
}

#[derive(Debug)]
struct Parser<'a, 'b, I: Iterator<Item=Token<'a, 'b>>> {
    iter: Peekable<I>
}

pub fn parse<'a, 'b, I>(iter: I) -> Production<'a, 'b>
where I: Iterator<Item=Token<'a, 'b>> {
    let peekable = iter.peekable();
    let mut parser = Parser { iter: peekable };
    parser.parse_s()
}

impl <'a, 'b> Production<'a, 'b> {
    pub fn new_terminal(tok: Token<'a, 'b>) -> Self {
        Production {
            ty: ProductionType::Terminal,
            children: vec![Branch::Terminal(tok)]
        }
    }

    pub fn new_nonterminal(ty: ProductionType) -> Self {
        const DEFAULT_CAPACITY: usize = 4;
        Production {
            ty: ty,
            children: Vec::with_capacity(DEFAULT_CAPACITY)
        }
    }

    pub fn push_production(&mut self, prod: Production<'a, 'b>) {
        self.children.push(Branch::Nonterminal(prod));
    }
}

impl <'a, 'b, I: Iterator<Item=Token<'a, 'b>>> Parser<'a, 'b, I> {
    pub fn pop_token(&mut self) -> Token<'a, 'b> {
        self.iter.next().unwrap()
    }

    pub fn peek_type(&mut self) -> TokenType {
        self.iter.peek()
            .map(|ref tok| tok.get_type())
            .unwrap_or(TokenType::EOF)
    }

    pub fn parse_s(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_s());
        let mut s = Production::new_nonterminal(ProductionType::S);
        if self.peek_sws() {
            s.push_production(self.parse_sws());
        }
        s.push_production(self.parse_blocks());
        s.push_production(Production::new_terminal(self.pop_token()));
        s
    }

    pub fn peek_s(&mut self) -> bool {
        self.peek_sws()
        || self.peek_blocks()
    }

    pub fn parse_blocks(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_blocks());
        unimplemented!()
    }

    pub fn peek_blocks(&mut self) -> bool {
        self.peek_block()
    }

    pub fn parse_block(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_block());
        unimplemented!()
    }

    pub fn peek_block(&mut self) -> bool {
        self.peek_text()
        || self.peek_comment()
        || self.peek_expr()
        || self.peek_math()
    }

    pub fn parse_text(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_text());
        unimplemented!()
    }

    pub fn peek_text(&mut self) -> bool {
        self.peek_text_item()
    }

    pub fn parse_text_item(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_text_item());
        unimplemented!()
    }

    pub fn peek_text_item(&mut self) -> bool {
        match self.peek_type() {
            TokenType::Word
            | TokenType::Char
            | TokenType::Number
            | TokenType::Escaped,
            | TokenType::KeyStart
            | TokenType::Quote => true,
            _ => false
        }
    }

    pub fn parse_comment(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_comment());
        unimplemented!()
    }

    pub fn peek_comment(&mut self) -> bool {
        self.peek_type() == TokenType::OpenComment
    }

    pub fn parse_comment_body(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_comment_body());
        unimplemented!()
    }

    pub fn peek_comment_body(&mut self) -> bool {
        self.peek_comment() || self.peek_comment_term()
    }

    pub fn parse_comment_term(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_comment_term());
        unimplemented!()
    }

    pub fn peek_comment_term(&mut self) -> bool {
        match self.peek_type() {
            TokenType::EOF
            | TokenType::OpenComment
            | TokenType::CloseComment => false,
            _ => true
        }
    }

    pub fn parse_expr(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_expr());
        unimplemented!()
    }

    pub fn peek_expr(&mut self) -> bool {
        self.peek_type() == TokenType::OpenExpression
    }

    pub fn parse_expr_body(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_expr_body());
        unimplemented!()
    }

    pub fn peek_expr_body(&mut self) -> bool {
        self.peek_aws()
        || self.peek_expr_item()
        || self.peek_expr_prop()
    }

    pub fn parse_expr_item(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_expr_item());
        unimplemented!()
    }

    pub fn peek_expr_item(&mut self) -> bool {
        self.peek_expr_ident()
        || self.peek_expr_literal()
        || self.peek_comment()
        || self.peek_expr()
        || self.peek_math()
    }

    pub fn parse_expr_prop(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_expr_prop());
        unimplemented!()
    }

    pub fn peek_expr_prop(&mut self) -> bool {
        self.peek_type() == TokenType::KeyStart
    }

    pub fn parse_expr_ident(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_expr_ident());
        unimplemented!()
    }

    pub fn peek_expr_ident(&mut self) -> bool {
        match self.peek_type() {
            TokenType::Char
            | TokenType::Word => true,
            _ => false
        }
    }

    pub fn parse_expr_literal(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_expr_literal());
        unimplemented!()
    }

    pub fn peek_expr_literal(&mut self) -> bool {
        self.peek_string()
        || self.peek_type == TokenType::Number
    }

    pub fn parse_math(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_math());
        unimplemented!()
    }

    pub fn peek_math(&mut self) -> bool {
        self.peek_type() == TokenType::OpenMath
    }

    pub fn parse_math_body(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_math_body());
        unimplemented!()
    }

    pub fn peek_math_body(&mut self) -> bool {
        self.peek_aws() || self.peek_math_term()
    }

    pub fn parse_math_term(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_math_term());
        unimplemented!()
    }

    pub fn peek_math_term(&mut self) -> bool {
        self.peek_comment()
        || self.peek_expr()
        || self.peek_math()
        || match self.peek_type() {
            TokenType::Char
            | TokenType::Escaped
            | TokenType::KeyStart
            | TokenType::Number
            | TokenType::Word => true,
            _ => false
        }
    }

    pub fn parse_string(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_string());
        unimplemented!()
    }

    pub fn peek_string(&mut self) -> bool {
        self.peek_type() == TokenType::Quote
    }

    pub fn parse_string_body(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_string_body());
        let mut body = Production::new_nonterminal(ProductionType::StringBody);
        body.push_production(self.parse_string_term());
        while self.peek_string_term() {
            body.push_production(self.parse_string_term());
        }
        body
    }

    pub fn peek_string_body(&mut self) -> bool {
        self.peek_string_term()
    }

    pub fn parse_string_term(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_string_term());
        Production::new_terminal(self.pop_token())
    }

    pub fn peek_string_term(&mut self) -> bool {
        match self.peek_type() {
            TokenType::Quote
            | TokenType::EOF => false,
            _ => true
        }
    }

    pub fn parse_aws(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_aws());
        let mut aws = Production::new_nonterminal(ProductionType::AnyWhiteSpace);
        aws.push_production(Production::new_terminal(self.pop_token()));
        if self.peek_aws() {
            aws.push_production(self.parse_aws());
        }
        aws
    }

    pub fn peek_aws(&mut self) -> bool {
        match self.peek_type() {
            TokenType::BlankLine
            | TokenType::NewLine
            | TokenType::Space => true,
            _ => false
        }
    }

    pub fn parse_sws(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_sws());
        let mut sws = Production::new_nonterminal(ProductionType::SigWhiteSpace);
        sws.push_production(Production::new_terminal(self.pop_token()));
        if self.peek_ws() {
            sws.push_production(self.parse_ws());
        }
        sws
    }

    pub fn peek_sws(&mut self) -> bool {
        self.peek_type() == TokenType::BlankLine
    }

    pub fn parse_ws(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_ws());
        let mut ws = Production::new_nonterminal(ProductionType::WhiteSpace);
        ws.push_production(Production::new_terminal(self.pop_token()));
        if self.peek_ws() {
            ws.push_production(self.parse_ws());
        }
        ws
    }

    pub fn peek_ws(&mut self) -> bool {
        match self.peek_type() {
            TokenType::NewLine
            | TokenType::Space => true,
            _ => false
        }
    }
}
