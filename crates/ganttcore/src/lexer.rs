//! Lexer: splits input into lines and tokens, handling comments
//! (`'` line comments like PlantUML) and bracketed task names.

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Word(String),
    Number(f64),
    Date(String),
    Percent(f64),
    LBracket,
    RBracket,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub tok: Tok,
    /// 1-based line number for error messages.
    pub line: usize,
}

pub fn strip_comment(line: &str) -> &str {
    let trimmed = line.trim_start();
    if trimmed.starts_with('\'') {
        ""
    } else {
        line
    }
}

pub fn tokenize_line(line: &str, line_no: usize) -> Vec<Token> {
    let mut tokens = Vec::new();
    let bytes: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        match c {
            '[' => {
                tokens.push(Token { tok: Tok::LBracket, line: line_no });
                i += 1;
            }
            ']' => {
                tokens.push(Token { tok: Tok::RBracket, line: line_no });
                i += 1;
            }
            _ => {
                // word / number / date / percent
                let start = i;
                while i < bytes.len() && !bytes[i].is_whitespace() && bytes[i] != '[' && bytes[i] != ']' {
                    i += 1;
                }
                let word: String = bytes[start..i].iter().collect();
                tokens.push(Token { tok: classify(&word), line: line_no });
            }
        }
    }
    tokens
}

fn classify(word: &str) -> Tok {
    // Date: YYYY-MM-DD
    let dateish = word.len() == 10
        && word.as_bytes()[4] == b'-'
        && word.as_bytes()[7] == b'-'
        && word[..4].chars().all(|c| c.is_ascii_digit())
        && word[5..7].chars().all(|c| c.is_ascii_digit())
        && word[8..].chars().all(|c| c.is_ascii_digit());
    if dateish {
        return Tok::Date(word.to_string());
    }
    if word.ends_with('%') {
        let num: String = word[..word.len() - 1].chars().filter(|c| !c.is_whitespace()).collect();
        if let Ok(v) = num.parse::<f64>() {
            return Tok::Percent(v);
        }
    }
    if let Ok(v) = word.parse::<f64>() {
        return Tok::Number(v);
    }
    Tok::Word(word.to_string())
}

/// Tokenize a whole document.
pub fn tokenize(src: &str) -> Vec<Token> {
    let mut out = Vec::new();
    for (idx, raw) in src.lines().enumerate() {
        let line = strip_comment(raw);
        if line.trim().is_empty() {
            continue;
        }
        if line.trim_start().starts_with('@') {
            // @startgantt / @endgantt markers
            let w = line.trim();
            out.push(Token { tok: Tok::Word(w.to_string()), line: idx + 1 });
            continue;
        }
        out.extend(tokenize_line(line, idx + 1));
    }
    out
}
