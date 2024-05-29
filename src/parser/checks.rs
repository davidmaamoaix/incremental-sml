use phf::{phf_set, Set};

static RESERVED_KW: Set<&'static str> = phf_set! {
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
    "where"
};

// Note: a lot of these listed operators cannot even be validly formed as they
// contains symbols outside of the set of symbols that are legal characters of
// a user-defined operator. However, they will still be included in this list
// as `sml97-defn` specified them as "reserved words".
static RESERVED_OP: Set<&'static str> = phf_set! {
    "(", ")", "[", "]", "{", "}", ",", ":", ";",
    "...", "_", "|", "=", "=>", "->", "#", ":>"
};

pub fn is_keyword(s: &str) -> bool {
    RESERVED_KW.contains(s)
}

pub fn is_reserved_op(s: &str) -> bool {
    RESERVED_OP.contains(s)
}
