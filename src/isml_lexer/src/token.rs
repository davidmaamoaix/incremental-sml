use string_interner::symbol::SymbolU32;

use isml_atom::span::Span;
use crate::errors::Error;
use crate::keywords::Kw;

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
    Keyword(Kw),
    Ignore
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

pub fn skip_comment(ptr: &mut usize, code: &[u8]) -> Result<bool, Error> {

    // Guard to ensure that there is a comment; the loop lexing logic is based
    // on this premise (e.g., if `skip_comment` gets called on the prefix "*)",
    // then the loop will behave incorrectly).
    let fst = code.get(*ptr);
    let snd = code.get(*ptr + 1);
    if fst != Some(&b'(') || snd != Some(&b'*') {
        return Ok(false);
    }

    *ptr += 2;
    let mut depth: usize = 1;
    loop {
        match (code.get(*ptr), code.get(*ptr + 1)) {
            (Some(b'('), Some(b'*')) => {
                *ptr += 2;
                depth += 1;
            },
            (Some(b'*'), Some(b')')) => {
                *ptr += 2;
                depth -= 1;
                if (depth == 0) {
                    return Ok(true);
                }
            },
            (Some(_), _) => *ptr += 1,
            (None, _) => return Err(Error::UnclosedComment),
        }
    }
}

// On successful parse, increment `ptr` and return the token type.
// On failed parse, return an error.
pub fn step<'a>(ptr: &mut usize, code: &'a [u8]) -> Result<TkContent<'a>, Error> {
    // Comment.
    if skip_comment(ptr, code)? {
        return Ok(TkContent::Ignore);
    }

    // Identifier.
    todo!();
}

pub fn lex(path: SymbolU32, code: &[u8]) -> Result<Vec<Token>, LexError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut ptr: usize = 0;

    while let Some(&b) = code.get(ptr) {
        skip_whitespace(&mut ptr, code);

        // TODO: lex
        let start = ptr;
    }

    Ok(tokens)
}
