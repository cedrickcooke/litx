extern crate litx;

const _MINIMAL_TEST: &'static str = r#"{}"#;

const _LITTLE_TEST: &'static str =
r#"
{litx :author "Cedrick Cooke"}

{frobnicate :foo "bar"}
Same paragraph: {$t$}.

New paragraph.
"#;

const _BIG_TEST: &'static str =
r#"{!
    This is a simple, example litx document.
    Hopefully, this doesn't trip up the close comment: \!}
!}
{litx
    :doctype mla
    :title "Example Document"
    :author "Cedrick Cooke"}

{litx.meta :table-of-contents none}

{h1 "Introduction"}
This is a plaintext paragraph.
These lines have not been separated by a blank line.
Lorem ipsum dolor sit amet.

This line is separated by a blank line on either side.

{h2 "Sub-header"}
On this line we have some funky styling, like inline math: {$1+2$}, and {emph "italics"}.
There is also an escaped newline: \n, outside of an expression.
Even worse, what if we have a newline in a word? Foo\nbar.

{h2 "Danger zone"}
I think this might actually break things. Is this fine!? What about this: $25.

{ignore "This is a string which contains {! a comment !}"}

{! This is another comment at the end. It contains "a string" !}"#;

fn main() {
    let tokens = litx::lex::Lexer::new(_LITTLE_TEST);
    let tree = litx::parse::parse(tokens);
    tree.write_graphviz(&mut std::io::stdout());
}
