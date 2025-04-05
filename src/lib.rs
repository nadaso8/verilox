/*
    A primary goal of the Verilox language frontend for the Verilog Hardware Description Language is to provide a
    library which is able to be maintained long term and is easily audited for adherance to IEEE standard 1800-2017.
    To this end it is imperative that we provide rigorous documentation and links back to the original standard document
    in order to easily cross reference each portion of the code base for accuracy.

    Each data strucutre which implements the AST trait must provide in its doc comment a reference to the standard formatted
    as follows.

    ```<Section Header> DEF: <Definition Index>```

    where:
        <Section Header> is the Header Prior to that series of definitions contained in annex A of IEEE std 1364-2005.
        <Definition Index> is the 0 indexed position of that definition within the section.
*/

use nom::{
    self,
    branch::alt,
    bytes::streaming::tag,
    character::streaming::{alpha1, alphanumeric1},
    combinator::recognize,
    multi::many0_count,
    sequence::pair,
    IResult, Parser,
};

pub trait AST: Sized {
    fn gen_ast(i: &str) -> nom::IResult<&str, Self, ParserErr>;
}

#[derive(Debug)]
struct ASTNodeLocation {
    line: usize,
    head: usize,
    tail: usize,
    file: std::path::PathBuf,
}

enum ParserErr {
    //A.9.4 White space DEF: 0
    WhiteSpace,
    SimpleIdentifier,
    EscapedIdentifier,
}

/// A.1.2 SystemVerilog source text DEF: 0
#[derive(Debug)]
pub struct SourceText {}

impl AST for SourceText {
    fn gen_ast(i: &str) -> nom::IResult<&str, Self, ParserErr> {
        todo!()
    }
}

/// A.9.3 Identifiers DEF:
#[derive(Debug)]
struct SimpleIdentifier {
    body: Box<str>,
}

impl AST for SimpleIdentifier {
    fn gen_ast(i: &str) -> nom::IResult<&str, Self, ParserErr> {
        todo!()
    }
}

/// A.9.3 Identifiers DEF:
#[derive(Debug)]
struct EscapedIdentifier {
    body: Box<str>,
}

impl AST for EscapedIdentifier {
    fn gen_ast(i: &str) -> nom::IResult<&str, Self, ParserErr> {
        todo!()
    }
}

/// A.9.4 White space DEF: 0
///
/// Note that while the formal syntax contains EOF as a success
/// state for its definition of whitespace this is conflicting
/// with section 5.3's defition which makes no mention ofEOF.
/// I have decided to not include it in my implementation
/// since it could lead to various undesierable edge cases such
/// infinite loops when parsing empty files.
#[derive(Debug)]
enum WhiteSpace {
    Space,
    Tab,
    Newline,
}

impl AST for WhiteSpace {
    fn gen_ast(i: &str) -> nom::IResult<&str, Self, ParserErr> {
        let (consumed, remainder) = match i.len() {
            0 => return IResult::Err(nom::Err::Incomplete(nom::Needed::new(1))),
            1 => (i, ""),
            _ => i.split_at(1),
        };

        match consumed {
            " " => IResult::Ok((remainder, WhiteSpace::Space)),
            "\t" => IResult::Ok((remainder, WhiteSpace::Tab)),
            "\n" => IResult::Ok((remainder, WhiteSpace::Newline)),
            _ => IResult::Err(nom::Err::Error(ParserErr::WhiteSpace)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_space() {
        todo!()
    }

    #[test]
    fn simple_identifier() {
        todo!()
    }

    #[test]
    fn escaped_identifier() {
        todo!()
    }
}
