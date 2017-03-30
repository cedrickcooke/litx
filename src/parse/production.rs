use ::lex::Token;
use ::graphviz::Graphviz;
use super::ProductionType;
use std::io;

static mut PRODUCTION_ID: u64 = 0;

#[derive(Debug)]
pub struct Production<'a, 'b> {
    ty: ProductionType,
    id: u64,
    children: Vec<Branch<'a, 'b>>
}

#[derive(Debug)]
pub enum Branch<'a, 'b> {
    Terminal(Token<'a, 'b>),
    Nonterminal(Production<'a, 'b>)
}

impl <'a, 'b> Production<'a, 'b> {
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

impl <'a, 'b, W: io::Write> Graphviz<W> for Production<'a, 'b> {
    fn get_vertex_name(&self) -> String {
        format!("{:?}_{}", self.ty, self.id)
    }

    fn write_edges(&self, writer: &mut W) -> io::Result<()> {
        let name = Graphviz::<W>::get_vertex_name(self);
        for child in &self.children {
            match *child {
                Branch::Terminal(ref token) => {
                    writeln!(writer, "\t{} -> {};", name, Graphviz::<W>::get_vertex_name(token))?;
                },
                Branch::Nonterminal(ref production) => {
                    writeln!(writer, "\t{} -> {};", name, Graphviz::<W>::get_vertex_name(production))?;
                    production.write_edges(writer)?;
                }
            }
        }
        Ok(())
    }
}
