#![allow(dead_code)]

use ::lex::Token;
use ::lex::TokenType;
use std::io::Write;
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

static mut PRODUCTION_ID: u64 = 0;

#[derive(Debug)]
pub struct Production<'a, 'b> {
    ty: ProductionType,
    id: u64,
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
    pub fn write_graphviz<W: Write>(&self, writer: &mut W) {
        writeln!(writer, "strict digraph {{").unwrap();
        self.write_graphviz_item(writer);
        writeln!(writer, "}}").unwrap();
    }

    fn write_graphviz_item<W: Write>(&self, writer: &mut W) {
        let name = self.get_name();
        for child in self.children.iter() {
            match *child {
                Branch::Terminal(ref token) => {
                    writeln!(writer, "\t{} -> {};", name, token.get_graphviz_name()).unwrap();
                },
                Branch::Nonterminal(ref production) => {
                    writeln!(writer, "\t{} -> {};", name, production.get_name()).unwrap();
                    production.write_graphviz_item(writer);
                }
            }
        }
    }

    fn get_name(&self) -> String {
        format!("{:?}_{}", self.ty, self.id)
    }
}

impl <'a, 'b> Production<'a, 'b> {
    pub fn new_terminal(tok: Token<'a, 'b>) -> Self {
        Production {
            ty: ProductionType::Terminal,
            id: unsafe {
                let tmp = PRODUCTION_ID;
                PRODUCTION_ID += 1;
                tmp
            },
            children: vec![Branch::Terminal(tok)]
        }
    }

    pub fn new_nonterminal(ty: ProductionType) -> Self {
        const DEFAULT_CAPACITY: usize = 4;
        Production {
            ty: ty,
            id: unsafe {
                let tmp = PRODUCTION_ID;
                PRODUCTION_ID += 1;
                tmp
            },
            children: Vec::with_capacity(DEFAULT_CAPACITY)
        }
    }

    pub fn push(&mut self, bran: Branch<'a, 'b>) {
        self.children.push(bran);
    }

    pub fn push_production(&mut self, prod: Production<'a, 'b>) {
        self.children.push(Branch::Nonterminal(prod));
    }

    pub fn push_terminal(&mut self, tok: Token<'a, 'b>) {
        self.children.push(Branch::Terminal(tok));
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
        if self.peek_aws() {
            s.push_production(self.parse_aws());
        }
        s.push_production(self.parse_blocks());
        s.push_terminal(self.pop_token());
        s
    }

    pub fn peek_s(&mut self) -> bool {
        self.peek_aws() || self.peek_blocks()
    }

    pub fn parse_blocks(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_blocks());
        let mut blocks = Production::new_nonterminal(ProductionType::Blocks);
        while self.peek_blocks() {
            blocks.push_production(self.parse_block());
            if self.peek_sws() {
                blocks.push_production(self.parse_sws());
            } else {
                break;
            }
        }
        blocks
    }

    pub fn peek_blocks(&mut self) -> bool {
        self.peek_block()
    }

    pub fn parse_block(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_block());
        let mut block = Production::new_nonterminal(ProductionType::Block);
        while self.peek_block() {
            if self.peek_text() {
                block.push_production(self.parse_text());
            } else {
                if self.peek_comment() {
                    block.push_production(self.parse_comment());
                } else if self.peek_expr() {
                    block.push_production(self.parse_expr());
                } else {
                    assert!(self.peek_math());
                    block.push_production(self.parse_math());
                }
                if self.peek_ws() {
                    block.push_production(self.parse_ws());
                }
            }
        }
        block
    }

    pub fn peek_block(&mut self) -> bool {
        self.peek_text()
        || self.peek_comment()
        || self.peek_expr()
        || self.peek_math()
    }

    pub fn parse_text(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_text());
        let mut text = Production::new_nonterminal(ProductionType::Text);
        while self.peek_ws() || self.peek_text_item() {
            if self.peek_text() {
                text.push(self.parse_text_item());
            }
            if self.peek_ws() {
                text.push_production(self.parse_ws());
            }
        }
        text
    }

    pub fn peek_text(&mut self) -> bool {
        self.peek_text_item()
    }

    pub fn parse_text_item(&mut self) -> Branch<'a, 'b> {
        assert!(self.peek_text_item());
        Branch::Terminal(self.pop_token())
    }

    pub fn peek_text_item(&mut self) -> bool {
        match self.peek_type() {
            TokenType::Word
            | TokenType::Char
            | TokenType::Number
            | TokenType::Escaped
            | TokenType::KeyStart
            | TokenType::Quote => true,
            _ => false
        }
    }

    pub fn parse_comment(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_comment());
        let mut comment = Production::new_nonterminal(ProductionType::Comment);
        comment.push_terminal(self.pop_token());
        if self.peek_comment_body() {
            comment.push_production(self.parse_comment_body());
        }
        assert!(self.peek_type() == TokenType::CloseComment);
        comment.push_terminal(self.pop_token());
        comment
    }

    pub fn peek_comment(&mut self) -> bool {
        self.peek_type() == TokenType::OpenComment
    }

    pub fn parse_comment_body(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_comment_body());
        let mut body = Production::new_nonterminal(ProductionType::CommentBody);
        while self.peek_comment_body() {
            if self.peek_comment() {
                body.push_production(self.parse_comment());
            } else {
                assert!(self.peek_comment_term());
                body.push(self.parse_comment_term());
            }
        }
        body
    }

    pub fn peek_comment_body(&mut self) -> bool {
        self.peek_comment() || self.peek_comment_term()
    }

    pub fn parse_comment_term(&mut self) -> Branch<'a, 'b> {
        assert!(self.peek_comment_term());
        Branch::Terminal(self.pop_token())
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
        let mut expr = Production::new_nonterminal(ProductionType::Expr);
        expr.push_terminal(self.pop_token());
        if self.peek_expr_body() {
            expr.push_production(self.parse_expr_body());
        }
        assert!(self.peek_type() == TokenType::CloseExpression);
        expr.push_terminal(self.pop_token());
        expr
    }

    pub fn peek_expr(&mut self) -> bool {
        self.peek_type() == TokenType::OpenExpression
    }

    pub fn parse_expr_body(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_expr_body());
        let mut body = Production::new_nonterminal(ProductionType::ExprBody);
        while self.peek_expr_body() {
            if self.peek_aws() {
                body.push_production(self.parse_aws())
            } else if self.peek_expr_item() {
                body.push(self.parse_expr_item());
            } else {
                assert!(self.peek_expr_prop());
                body.push_production(self.parse_expr_prop());
            }
        }
        body
    }

    pub fn peek_expr_body(&mut self) -> bool {
        self.peek_aws()
        || self.peek_expr_item()
        || self.peek_expr_prop()
    }

    pub fn parse_expr_item(&mut self) -> Branch<'a, 'b> {
        assert!(self.peek_expr_item());
        if self.peek_expr_ident() {
            Branch::Nonterminal(self.parse_expr_ident())
        } else if self.peek_expr_literal() {
            self.parse_expr_literal()
        } else if self.peek_comment() {
            Branch::Nonterminal(self.parse_comment())
        } else if self.peek_expr() {
            Branch::Nonterminal(self.parse_expr())
        } else {
            assert!(self.peek_math());
            Branch::Nonterminal(self.parse_math())
        }
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
        let mut prop = Production::new_nonterminal(ProductionType::ExprProp);
        prop.push_terminal(self.pop_token());
        prop.push_production(self.parse_expr_ident());
        prop.push_production(self.parse_aws());
        prop.push(self.parse_expr_item());
        prop
    }

    pub fn peek_expr_prop(&mut self) -> bool {
        self.peek_type() == TokenType::KeyStart
    }

    pub fn parse_expr_ident(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_expr_ident());
        let mut ident = Production::new_nonterminal(ProductionType::ExprIdent);
        while self.peek_expr_ident() {
            ident.push_terminal(self.pop_token());
        }
        ident
    }

    pub fn peek_expr_ident(&mut self) -> bool {
        match self.peek_type() {
            TokenType::Char
            | TokenType::Word => true,
            _ => false
        }
    }

    pub fn parse_expr_literal(&mut self) -> Branch<'a, 'b> {
        assert!(self.peek_expr_literal());
        if self.peek_string() {
            Branch::Nonterminal(self.parse_string())
        } else {
            assert!(self.peek_type() == TokenType::Number);
            Branch::Terminal(self.pop_token())
        }
    }

    pub fn peek_expr_literal(&mut self) -> bool {
        self.peek_string()
        || self.peek_type() == TokenType::Number
    }

    pub fn parse_math(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_math());
        let mut math = Production::new_nonterminal(ProductionType::Math);
        math.push_terminal(self.pop_token());
        if self.peek_math_body() {
            math.push_production(self.parse_math_body());
        }
        assert!(self.peek_type() == TokenType::CloseMath);
        math.push_terminal(self.pop_token());
        math
    }

    pub fn peek_math(&mut self) -> bool {
        self.peek_type() == TokenType::OpenMath
    }

    pub fn parse_math_body(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_math_body());
        let mut body = Production::new_nonterminal(ProductionType::MathBody);
        if self.peek_aws() {
            body.push_production(self.parse_aws());
        } else {
            assert!(self.peek_math_term());
            body.push(self.parse_math_term());
        }
        if self.peek_math_body() {
            body.push_production(self.parse_math_body());
        }
        body
    }

    pub fn peek_math_body(&mut self) -> bool {
        self.peek_aws() || self.peek_math_term()
    }

    pub fn parse_math_term(&mut self) -> Branch<'a, 'b> {
        assert!(self.peek_math_term());
        if self.peek_comment() {
            Branch::Nonterminal(self.parse_comment())
        } else if self.peek_expr() {
            Branch::Nonterminal(self.parse_expr())
        } else if self.peek_math() {
            Branch::Nonterminal(self.parse_math())
        } else {
            Branch::Terminal(self.pop_token())
        }
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
        let mut string = Production::new_nonterminal(ProductionType::String);
        string.push_terminal(self.pop_token());
        if self.peek_string_body() {
            string.push_production(self.parse_string_body());
        }
        assert!(self.peek_type() == TokenType::Quote);
        string.push_terminal(self.pop_token());
        string
    }

    pub fn peek_string(&mut self) -> bool {
        self.peek_type() == TokenType::Quote
    }

    pub fn parse_string_body(&mut self) -> Production<'a, 'b> {
        assert!(self.peek_string_body());
        let mut body = Production::new_nonterminal(ProductionType::StringBody);
        while self.peek_string_term() {
            body.push(self.parse_string_term());
        }
        body
    }

    pub fn peek_string_body(&mut self) -> bool {
        self.peek_string_term()
    }

    pub fn parse_string_term(&mut self) -> Branch<'a, 'b> {
        assert!(self.peek_string_term());
        Branch::Terminal(self.pop_token())
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
        aws.push_terminal(self.pop_token());
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
        sws.push_terminal(self.pop_token());
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
        ws.push_terminal(self.pop_token());
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
