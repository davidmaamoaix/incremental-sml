use string_interner::symbol::SymbolU32;

use crate::atom::span::Span;

// The types of literals. Note that a string literal might not match the
// corresponding sequence in the source file due to preprocessing, so they get
// copied.
pub enum LiteralType {
    Num(i64),
    Word(i16),
    Float(f64),
    Char(char),
    Str(String)
}

pub enum TkContent<'a> {
    ID(&'a str),
    Literal(LiteralType),
    Keyword(&'a str),
}

pub struct Token<'a> {
    content: TkContent<'a>,
    span: Span
}

pub struct LexError {
    err: String,
    offset: usize
}

pub fn advance_while<P>(ptr: &mut usize, code: &[u8], pred: P)
where
    P: Fn(u8) -> bool
{
    while let Some(&b) = code.get(*ptr) {
        if !pred(b) {
            return;
        }

        *ptr += 1;
    }
}

pub fn skip_whitespace(ptr: &mut usize, code: &[u8]) {
    advance_while(ptr, code, |c| c.is_ascii_whitespace() || c == 0xb);
}

pub fn lex(path: SymbolU32, code: &[u8]) -> Result<Vec<Token>, LexError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut ptr: usize = 0;

    while let Some(&b) = code.get(ptr) {
        skip_whitespace(&mut ptr, code);

        // TODO: lex
    }

    Ok(tokens)
}
