use phf::{phf_set, Set};

use isml_macros::gen_keyword_defs;

// static RESERVED_KW: Set<&'static str> = phf_set! {

gen_keyword_defs! {
    "abstype",
    "and",
    "andalso",
    "as",
    "case",
    "datatype",
    "do",
    "else",
    "end",
    "exception",
    "fn",
    "fun",
    "handle",
    "if",
    "in",
    "infix",
    "infixr",
    "let",
    "local",
    "nonfix",
    "of",
    "op",
    "open",
    "orelse",
    "raise",
    "rec",
    "then",
    "type",
    "val",
    "with",
    "withtype",
    "while",
    "eqtype",
    "functor",
    "include",
    "sharing",
    "sig",
    "signature",
    "struct",
    "structure",
    "where",
    "(": LPAREN,
    ")": RPAREN,
    "[": LBRACK,
    "]": RBRACK,
    "{": LBRACE,
    "}": RBRACE,
    ",": COMMA,
    ":": COLON,
    ";": SCOLON,
    "...": ELLIPS,
    "_": USCORE,
    "|": VBAR,
    "=>": DARROW,
    "->": ARROW,
    "#": HASH,
    ":>": ASCRIB
}

pub fn is_keyword(s: &str) -> bool {
    // RESERVED_KW.contains(s)
    false
}
