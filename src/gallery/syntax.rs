//! Small, dependency-free Rust highlighter for the read-only source panel.
//!
//! This is lexical highlighting rather than a Rust parser. Unrecognized syntax
//! stays in the normal text color; the original source is never modified.
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    Keyword,
    String,
    Comment,
    Number,
    Type,
    Macro,
    Function,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Token {
    pub range: Range<usize>,
    pub kind: Kind,
}

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

fn char_len(source: &str, at: usize) -> usize {
    source[at..].chars().next().unwrap().len_utf8()
}

fn quoted_end(source: &str, quote: usize, delimiter: u8) -> usize {
    let bytes = source.as_bytes();
    let mut i = quote + 1;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => {
                i += 1;
                if i < bytes.len() {
                    i += char_len(source, i);
                }
            }
            b if b == delimiter => return i + 1,
            _ => i += char_len(source, i),
        }
    }
    i
}

fn raw_string_end(source: &str, start: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut i = start;
    if matches!(bytes.get(i), Some(b'b' | b'c')) {
        i += 1;
    }
    if bytes.get(i) != Some(&b'r') {
        return None;
    }
    i += 1;
    let hash_start = i;
    while bytes.get(i) == Some(&b'#') {
        i += 1;
    }
    if bytes.get(i) != Some(&b'"') {
        return None;
    }
    let hashes = i - hash_start;
    i += 1;
    while i < bytes.len() {
        if bytes[i] == b'"'
            && i + 1 + hashes <= bytes.len()
            && bytes[i + 1..i + 1 + hashes].iter().all(|&b| b == b'#')
        {
            return Some(i + 1 + hashes);
        }
        i += char_len(source, i);
    }
    Some(i)
}

fn char_literal_end(source: &str, quote: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let i = quote + 1;
    if i >= bytes.len() || bytes[i] == b'\n' {
        return None;
    }
    if bytes[i] == b'\\' {
        // Escapes can include unicode braces; stop at the closing quote, never
        // crossing a line. Lifetimes do not start with a backslash.
        let end = quoted_end(source, quote, b'\'');
        return (end <= bytes.len()
            && bytes.get(end.wrapping_sub(1)) == Some(&b'\'')
            && !source[quote..end].contains('\n'))
        .then_some(end);
    }
    let end = i + char_len(source, i);
    (bytes.get(end) == Some(&b'\'')).then_some(end + 1)
}

pub(super) fn tokenize(source: &str) -> Vec<Token> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        let kind = if bytes[i..].starts_with(b"//") {
            i = source[i..].find('\n').map_or(bytes.len(), |n| i + n);
            Some(Kind::Comment)
        } else if bytes[i..].starts_with(b"/*") {
            i += 2;
            let mut depth = 1;
            while i < bytes.len() && depth > 0 {
                if bytes[i..].starts_with(b"/*") {
                    depth += 1;
                    i += 2;
                } else if bytes[i..].starts_with(b"*/") {
                    depth -= 1;
                    i += 2;
                } else {
                    i += char_len(source, i);
                }
            }
            Some(Kind::Comment)
        } else if let Some(end) = raw_string_end(source, i) {
            i = end;
            Some(Kind::String)
        } else if bytes[i] == b'"' {
            i = quoted_end(source, i, b'"');
            Some(Kind::String)
        } else if matches!(bytes[i], b'b' | b'c') && bytes.get(i + 1) == Some(&b'"') {
            i = quoted_end(source, i + 1, b'"');
            Some(Kind::String)
        } else if bytes[i] == b'\'' && char_literal_end(source, i).is_some() {
            i = char_literal_end(source, i).unwrap();
            Some(Kind::String)
        } else if bytes[i] == b'b'
            && bytes.get(i + 1) == Some(&b'\'')
            && char_literal_end(source, i + 1).is_some()
        {
            i = char_literal_end(source, i + 1).unwrap();
            Some(Kind::String)
        } else if bytes[i].is_ascii_digit() {
            i += 1;
            while i < bytes.len() {
                if bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' {
                    i += 1;
                } else if bytes[i] == b'.' && bytes.get(i + 1).is_some_and(u8::is_ascii_digit) {
                    i += 1;
                } else {
                    break;
                }
            }
            Some(Kind::Number)
        } else {
            let ch = source[i..].chars().next().unwrap();
            if ch == '_' || ch.is_alphabetic() {
                i += ch.len_utf8();
                while i < bytes.len() {
                    let next = source[i..].chars().next().unwrap();
                    if next == '_' || next.is_alphanumeric() {
                        i += next.len_utf8();
                    } else {
                        break;
                    }
                }
                let word = &source[start..i];
                if KEYWORDS.contains(&word) {
                    Some(Kind::Keyword)
                } else if bytes.get(i) == Some(&b'!') && bytes.get(i + 1) != Some(&b'=') {
                    Some(Kind::Macro)
                } else if ch.is_uppercase() {
                    Some(Kind::Type)
                } else if source[i..].trim_start_matches([' ', '\t']).starts_with('(') {
                    Some(Kind::Function)
                } else {
                    None
                }
            } else {
                i += ch.len_utf8();
                None
            }
        };
        if let Some(kind) = kind {
            tokens.push(Token {
                range: start..i,
                kind,
            });
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_common_rust_without_changing_source() {
        let source = "let title: String = format!(\"hello {}\", 42); show(title); // note";
        let found: Vec<_> = tokenize(source)
            .into_iter()
            .map(|t| (&source[t.range], t.kind))
            .collect();
        assert_eq!(
            found,
            vec![
                ("let", Kind::Keyword),
                ("String", Kind::Type),
                ("format", Kind::Macro),
                ("\"hello {}\"", Kind::String),
                ("42", Kind::Number),
                ("show", Kind::Function),
                ("// note", Kind::Comment)
            ]
        );
    }

    #[test]
    fn raw_strings_nested_comments_and_lifetimes_are_distinct() {
        let source = "/* outer /* inner */ end */ fn read<'a>(s: &'a str) { r#\"// not a comment\"#; '🦀'; }";
        let found: Vec<_> = tokenize(source)
            .into_iter()
            .map(|t| (&source[t.range], t.kind))
            .collect();
        assert_eq!(
            found,
            vec![
                ("/* outer /* inner */ end */", Kind::Comment),
                ("fn", Kind::Keyword),
                ("r#\"// not a comment\"#", Kind::String),
                ("'🦀'", Kind::String)
            ]
        );
    }
}
