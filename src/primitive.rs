#[derive(Clone, Debug)]
pub(crate) enum Primitive {
    Pop,
    Duplicate,
    Flip,
    Call,
    Join,
    Pair,
    Index,
    Print,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equals,
}

impl Primitive {
    pub(crate) fn from_char(c: char) -> Self {
        match c {
            '.' => Primitive::Pop,
            ':' => Primitive::Duplicate,
            '⭥' => Primitive::Flip,
            '!' => Primitive::Call,
            '”' => Primitive::Join,
            ',' => Primitive::Pair,
            '⤉' => Primitive::Index,
            '↗' => Primitive::Print,
            '+' => Primitive::Add,
            '-' => Primitive::Subtract,
            '×' => Primitive::Multiply,
            '÷' => Primitive::Divide,
            '=' => Primitive::Equals,
            _ => unreachable!(),
        }
    }
}
