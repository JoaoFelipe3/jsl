use super::result::*;
use std::fmt;

type Reader<'a> = std::iter::Peekable<std::str::Chars<'a>>;

// what were you expecting?
fn reader_from_string<'a>(s: &'a str) -> Reader<'a> {
    s.chars().peekable()
}

#[derive(Debug, Clone)]
enum TokenCategory {
    Identifier,
    Number,
    String,
    Symbol,
}

pub(crate) struct Token {
    category: TokenCategory,
    content: String,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: \"{}\"", self.category, self.content)
    }
}

const SYMBOLS: &str = "{}∅□.:⭥!”⤉↗→+-×÷=";
const DIGITS: &str = "0123456789";

// bulk of the logic
// yes this returns a RESULT OF AN OPTION 😭
fn read_a_token(reader: &mut Reader) -> JSLResult<Option<Token>> {
    if let Some(ch) = reader.next() {
        match ch {
            // symbol
            _ if SYMBOLS.contains(ch) => Ok(Some(Token {
                category: TokenCategory::Symbol,
                content: ch.into(),
            })),
            _ if DIGITS.contains(ch) => {
                // number
                let mut result: String = ch.into();
                let mut has_read_dot = false;
                while let Some(&d) = reader.clone().peek() {
                    if !(DIGITS.contains(d) || d == '.') {
                        break;
                    }
                    if d == '.' {
                        if has_read_dot {
                            break;
                        }
                        has_read_dot = true;
                    }
                    result.push(d);
                    reader.next();
                }
                Ok(Some(Token {
                    category: TokenCategory::Number,
                    content: result,
                }))
            }
            '"' => {
                // string
                let mut result: String = ch.into();
                let mut finished = false;
                while let Some(c) = reader.next() {
                    match c {
                        '\\' => {
                            result.push(c);
                            match reader.next() {
                                Some(e) => result.push(e),
                                None => break,
                            }
                        }
                        '"' => {
                            finished = true;
                            break;
                        }
                        _ => result.push(c),
                    }
                }
                if finished {
                    Ok(Some(Token {
                        category: TokenCategory::String,
                        content: result,
                    }))
                } else {
                    Err(JSLError {
                        msg: "unterminated string".into(),
                    })
                }
            }
            _ if ch.is_alphabetic() => {
                // identifiers
                let mut result: String = ch.into();
                while let Some(&l) = reader.clone().peek() {
                    if !l.is_alphabetic() {
                        break;
                    }
                    result.push(l);
                    reader.next();
                }
                Ok(Some(Token {
                    category: TokenCategory::Identifier,
                    content: result,
                }))
            }
            _ if ch.is_whitespace() => Ok(None),
            '#' => {
                // comments
                while let Some(i) = reader.next() {
                    if i == '\n' {
                        break;
                    }
                }
                Ok(None)
            }
            i => Ok(Some(Token {
                category: TokenCategory::Identifier,
                content: i.into(),
            })), // symbol identifiers
        }
    } else {
        Ok(None)
    }
}

pub fn tokenize(code: &str) -> JSLResult<Vec<Token>> {
    let mut reader = reader_from_string(code);
    let mut result: Vec<Token> = vec![];
    while reader.clone().peek().is_some() {
        // read tokens 'till there ain't any
        if let Some(token) = read_a_token(&mut reader)? {
            result.push(token);
        }
    }
    Ok(result)
}

// mmmm, no, very unwise
use super::{primitive::*, value::*};

#[derive(Clone)]
pub(crate) enum Statement {
    Binding(String),
    Identifier(String),
    Literal(Value), // functions actually make this have a circular import, lol
    Primitive(Primitive),
}

impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Binding(i) => write!(f, "→{i}"),
            Statement::Identifier(i) => write!(f, "{i}"),
            Statement::Literal(v) => write!(f, "{v}"),
            Statement::Primitive(p) => write!(f, "{p:?}"),
        }
    }
}

pub(crate) type AST = Vec<Statement>;

type TokenReader<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;

fn token_reader<'a>(tokens: &'a Vec<Token>) -> TokenReader<'a> {
    tokens.iter().peekable()
}

enum ParserContext {
    Global,
    Function,
}

// a better name for this would be parse_string
fn handle_escapes(string: &str) -> JSLResult<String> {
    let mut iter = string.chars();
    let mut result = String::new();
    while let Some(c) = iter.next() {
        match c {
            '"' => (), // do nothing, it's the outer quote
            // the real hassle
            '\\' => result.push(match iter.next().unwrap() {
                // unwrap will never fail --------⬏
                'r' => '\r', // goofy ahh windows
                'n' => '\n', // newline
                't' => '\t', // tab
                '"' => '"',  // quote
                // invalid escape!
                invalid => {
                    return Err(JSLError {
                        msg: format!("invalid escape sequence: \\{invalid}"),
                    });
                }
            }),
            _ => result.push(c), // just push it
        }
    }
    Ok(result)
}

fn parse_helper(reader: &mut TokenReader, context: ParserContext) -> JSLResult<AST> {
    let mut tree: AST = vec![];
    while let Some(token) = reader.next() {
        // very happy tuple destructuring
        match (token.category.clone(), token.content.clone().as_str()) {
            (TokenCategory::Number, number) => {
                // numbers: unwrap NEVER fails ---------------------------⬎
                tree.push(Statement::Literal(Value::Number(number.parse().unwrap())))
            }
            // identifiers
            (TokenCategory::Identifier, ident) => tree.push(Statement::Identifier(ident.into())),
            // null
            (TokenCategory::Symbol, "∅") => tree.push(Statement::Literal(Value::Null)),
            // empty list
            (TokenCategory::Symbol, "□") => tree.push(Statement::Literal(Value::List(vec![]))),
            // binding arrow
            (TokenCategory::Symbol, "→") => {
                if let Some(Token {
                    category: TokenCategory::Identifier,
                    content: ident,
                }) = reader.next()
                {
                    tree.push(Statement::Binding(ident.to_string()))
                } else {
                    return Err(JSLError {
                        msg: "expected identifier after →".into(),
                    });
                }
            }
            // open function
            (TokenCategory::Symbol, "{") => tree.push(Statement::Literal(Value::Function(
                // first-class functions 😭
                parse_helper(reader, ParserContext::Function)?,
            ))),
            // close function
            (TokenCategory::Symbol, "}") => match context {
                ParserContext::Function => return Ok(tree),
                _ => {
                    return Err(JSLError {
                        msg: "unexpected }".into(),
                    });
                }
            },
            // primitives
            (TokenCategory::Symbol, prim) => tree.push(Statement::Primitive(Primitive::from_char(
                prim.chars().next().unwrap(), // unwrap will NEVER EVER fail
            ))),
            // strings
            (TokenCategory::String, string) => {
                tree.push(Statement::Literal(Value::String(handle_escapes(string)?)))
            }
        }
    }
    // can only finish parsing when on a global context:
    match context {
        ParserContext::Global => Ok(tree),
        ParserContext::Function => Err(JSLError {
            msg: "expected } before eof".into(),
        }),
    }
}

pub(crate) fn parse(tokens: Vec<Token>) -> JSLResult<AST> {
    let mut reader = token_reader(&tokens);
    parse_helper(&mut reader, ParserContext::Global)
}
