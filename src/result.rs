#[derive(Debug)]
pub struct JSLError {
    pub msg: String,
}

/// yada yada yada result type
pub type JSLResult<T> = Result<T, JSLError>;
