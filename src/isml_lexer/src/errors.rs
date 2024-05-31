use std::fmt::Display;

pub enum Error {
    UnclosedComment,
    UnclosedParentheses,
    UnclosedStringLit
}