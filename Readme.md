# trim-margin: easy layouting of multi-line strings
[![Build Status](https://travis-ci.org/mindsbackyard/trim-margin.svg?branch=master)](https://travis-ci.org/mindsbackyard/trim-margin)
[![Crates.io](https://img.shields.io/crates/v/trim-margin.svg)](https://crates.io/crates/trim-margin)

This crate is intended to ease the use of multi-line strings in Rust.
When embedding strings with multiple lines in Rust all whitespaces, tabs, etc. are preserved even if they are just used for layouting one's code nicely.
```Rust
fn main() {
    println!("-----------------------");
    let misrepresented_multiline_string = "
        This is string
        spans over multiple lines,
        but its rendering preserves all whitespaces.

        Which is not what we usually intend in this case.
    ";
    println!("{}", misrepresented_multiline_string);
    println!("-----------------------");

    println!("-----------------------");
    let correctly_layouted_string = "For displaying
the a multiline strin properly
it would need to be layouted
like this.

Which is not very nice.";
    println!("{}", correctly_layouted_string);
    println!("-----------------------");
}
```

The `trim-margin` crate supports you with proper layouting.
```Rust
extern crate trim_margin;
use trim_margin::MarginTrimmable;

fn main() {
    let multiline_string_with_margin = "
        |This string has a margin
        |indicated by the '|' character.
        |
        |The following method call will remove ...
        | * a blank first/last line
        | * blanks before the margin prefix
        | * the margin prefix itself
    ".trim_margin();
    println!("{}", multiline_string_with_margin);
}
```
