mod parser;
mod production;
mod production_type;

use self::parser::Parser;
pub use self::production::Branch;
pub use self::production::Production;
pub use self::production_type::ProductionType;

pub fn parse<'a, 'b, I>(iter: I) -> Production<'a, 'b>
where I: Iterator<Item=::lex::Token<'a, 'b>> {
    Parser::new(iter).parse_s()
}
