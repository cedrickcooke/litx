# litx
Litx is a typesetting language with an emphasis on orthogonality.
Specifically, litx was designed as an alternative to LaTeX when working with documents that are less mathematically focused.
Although litx will have support for typesetting mathematics its most fitting use case is for prose, including essays and books.

Litx (from **lit**erary e**x**pressions) is both the name of the language and the reference compiler.

## Project Overview
Litx is implemented as a rust workspace, with the binary in the workspace root.
Check the book for a more detailed explanation of the library structure.

### Syntax Example
```
{litx
    :doctype article
    :title "Example Document"
    :authors {list "Cedrick Cooke"}}

{h1 "First section"} 
This is a simple paragraph.
{! Comments are block formatted, and {!nest!} correctly. !}
Math contexts are blocks too: {$ 5+3 $}.
Like many markup languages, blank lines are used to separate paragraphs.

This starts the second paragraph, for example.
```

## License
To be determined.
