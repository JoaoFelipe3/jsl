use super::{parse::*, primitive::*, result::*, value::*};
use std::{collections::HashMap, io::Write, io::stdout, iter::once};

pub(crate) fn gen_ast_from_code(code: &str) -> JSLResult<AST> {
    tokenize(code).and_then(parse) // quick little shortcut
}

pub(crate) fn run_ast(
    ast: AST,
    stack: &mut Vec<Value>,
    vars: &mut HashMap<String, Value>,
) -> JSLResult<()> {
    let mut iter = ast.into_iter().peekable();
    while let Some(statement) = iter.next() {
        match statement {
            Statement::Binding(id) => {
                // ok these ones have default values
                vars.insert(id, stack.pop().unwrap_or(Value::Null));
            }
            Statement::Literal(v) => stack.push(v),
            Statement::Identifier(id) => stack.push(vars.get(&id).unwrap_or(&Value::Null).clone()),
            // oooh boy!
            // and yes this handles EVERY PRIMITIVE in ONE FUNCTION so deal with this very long
            // match statement
            Statement::Primitive(p) => match p {
                Primitive::Pop => {
                    stack.pop();
                }
                Primitive::Duplicate => stack.push(stack.last().unwrap_or(&Value::Null).clone()),
                Primitive::Flip => match (stack.pop(), stack.pop()) {
                    (Some(x), Some(y)) => {
                        stack.push(x);
                        stack.push(y);
                    }
                    // do nothing if not enough stack values
                    (Some(x), _) => stack.push(x),
                    _ => (),
                },
                Primitive::Print => {
                    print!("{}", stack.pop().unwrap_or(Value::Null));
                    stdout().flush().or(Err(JSLError {
                        msg: "could not flush output :(".into(),
                    }))?; // ← needed because rust is dumb and doesn't flush output
                }
                Primitive::Call => match stack.pop().unwrap_or(Value::Null) {
                    Value::Function(ast) => {
                        if iter.clone().peek().is_some() {
                            run_ast(ast, stack, &mut (*vars).clone())?; // same stack reference,
                        // new vars reference
                        } else {
                            iter = ast.into_iter().peekable(); // tail recursion!
                        }
                    }
                    _ => {
                        return Err(JSLError {
                            msg: "invalid function".into(),
                        });
                    }
                },
                Primitive::Join => match (stack.pop(), stack.pop()) {
                    (Some(Value::String(a)), Some(Value::String(b))) => {
                        let mut res = a.clone();
                        res.push_str(b.as_str());
                        stack.push(Value::String(res));
                    }
                    (Some(Value::String(a)), Some(Value::Number(b))) => {
                        let mut res = a.clone();
                        res.push_str(format!("{b}").as_str());
                        stack.push(Value::String(res));
                    }
                    (Some(Value::Number(a)), Some(Value::String(b))) => {
                        let mut res = format!("{a}");
                        res.push_str(b.as_str());
                        stack.push(Value::String(res));
                    }
                    // compose 😈
                    (Some(Value::Function(a)), Some(Value::Function(b))) => {
                        stack.push(Value::Function(a.into_iter().chain(b).collect()));
                    }
                    // wrap a value in a list if you want lists to get joined as lists
                    (Some(Value::List(a)), Some(Value::List(b))) => {
                        stack.push(Value::List(a.into_iter().chain(b).collect()));
                    }
                    (Some(Value::List(a)), Some(b)) => {
                        stack.push(Value::List(a.into_iter().chain(once(b)).collect()));
                    }
                    (Some(a), Some(Value::List(b))) => {
                        stack.push(Value::List(once(a).chain(b).collect()));
                    }
                    (Some(x), Some(y)) => {
                        return Err(JSLError {
                            msg: format!("cannot join {} and {}", x.type_str(), y.type_str()),
                        });
                    }
                    _ => {
                        return Err(JSLError {
                            msg: "not enough values for ” join".into(),
                        });
                    }
                },
                Primitive::Index => {
                    match (stack.pop() /* index */, stack.pop() /* target */) {
                        (Some(Value::Number(i)), Some(Value::List(l))) => {
                            if i.fract() == 0.0 {
                                stack.push(
                                    l.get(if i < 0.0 {
                                        l.len() - (i as usize)
                                    } else {
                                        i as usize
                                    })
                                    .unwrap_or(&Value::Null)
                                    .clone(),
                                );
                            } else {
                                return Err(JSLError {
                                    msg: "expected integer index".into(),
                                });
                            }
                        }
                        (Some(Value::Number(i)), Some(Value::String(s))) => {
                            if i.fract() == 0.0 {
                                stack.push(
                                    s.chars()
                                        .nth(if i < 0.0 {
                                            s.len() - (i as usize)
                                        } else {
                                            i as usize
                                        })
                                        .map(|c| Value::String(c.into()))
                                        .unwrap_or(Value::Null),
                                );
                            } else {
                                return Err(JSLError {
                                    msg: "expected integer index".into(),
                                });
                            }
                        }
                        (Some(x), Some(y)) => {
                            return Err(JSLError {
                                msg: format!("cannot index {} with {}", y.type_str(), x.type_str()),
                            });
                        }
                        _ => {
                            return Err(JSLError {
                                msg: "not enough values for ⤉ index".into(),
                            });
                        }
                    }
                }
                Primitive::Add => match (stack.pop(), stack.pop()) {
                    (Some(Value::Number(x)), Some(Value::Number(y))) => {
                        stack.push(Value::Number(x + y))
                    }
                    (Some(x), Some(y)) => {
                        let use_join_hint = match (x.type_str(), y.type_str()) {
                            ("string", "string") => true,
                            ("list", "list") => true,
                            _ => false,
                        };
                        return Err(JSLError {
                            msg: format!(
                                "cannot add {} and {}{}",
                                x.type_str(),
                                y.type_str(),
                                if use_join_hint {
                                    ". perhaps you meant to use ” join?"
                                } else {
                                    ""
                                }
                            ),
                        });
                    }
                    _ => {
                        return Err(JSLError {
                            msg: "not enough values for + add".into(),
                        });
                    }
                },
                Primitive::Subtract => match (stack.pop(), stack.pop()) {
                    (Some(Value::Number(x)), Some(Value::Number(y))) => {
                        stack.push(Value::Number(y - x))
                    }
                    (Some(x), Some(y)) => {
                        return Err(JSLError {
                            msg: format!("cannot subtract {} from {}", x.type_str(), y.type_str()),
                        });
                    }
                    _ => {
                        return Err(JSLError {
                            msg: "not enough values for - subtract".into(),
                        });
                    }
                },
                Primitive::Multiply => match (stack.pop(), stack.pop()) {
                    (Some(Value::Number(x)), Some(Value::Number(y))) => {
                        stack.push(Value::Number(x * y))
                    }
                    (Some(x), Some(y)) => {
                        return Err(JSLError {
                            msg: format!("cannot multiply {} and {}", x.type_str(), y.type_str()),
                        });
                    }
                    _ => {
                        return Err(JSLError {
                            msg: "not enough values for × multiply".into(),
                        });
                    }
                },
                Primitive::Divide => match (stack.pop(), stack.pop()) {
                    (Some(Value::Number(x)), Some(Value::Number(y))) => {
                        stack.push(Value::Number(y / x))
                    }
                    (Some(x), Some(y)) => {
                        return Err(JSLError {
                            msg: format!("cannot divide {} by {}", y.type_str(), x.type_str()),
                        });
                    }
                    _ => {
                        return Err(JSLError {
                            msg: "not enough values for ÷ divide".into(),
                        });
                    }
                },
                Primitive::Equals => match (stack.pop(), stack.pop()) {
                    (Some(x), Some(y)) => stack.push(Value::Number((x == y).into())),
                    _ => {
                        return Err(JSLError {
                            msg: "not enough values for = equals".into(),
                        });
                    }
                },
                #[allow(unreachable_patterns)]
                _ => todo!(),
            },
        }
    }
    Ok(())
}
