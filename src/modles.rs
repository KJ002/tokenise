#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TokenType {
    Operand,
    Operator,
    Other,
    Null,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub content: String,
    pub token_type: TokenType,
}
