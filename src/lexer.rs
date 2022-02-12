use crate::modles::{Token, TokenType};

pub struct Lexer {
    pub content: String,
    pub operators: Vec<char>,
    pub ignores: Vec<char>,
    pub symbols: Vec<char>
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            content,
            operators: vec!['+', '-', '*', '/'],
            ignores: vec!['.'],
        }
    }

    pub fn tokenise(&self) -> Vec<Token> {
        let mut result: Vec<Token> = vec![];

        let mut buffer: Vec<char> = vec![];
        let mut current_buffer_type: TokenType = TokenType::Null;

        for character in self.content.chars() {
            let is_operand =
                character.is_digit(10) || (character == '.' && !buffer.iter().any(|x| *x == '.'));
            let is_operator = self.operators.iter().any(|x| *x == character);

            let token_type = if is_operand {
                TokenType::Operand
            } else if is_operator {
                TokenType::Operator
            } else {
                TokenType::Other
            };

            let token = Token {
                content: character.to_string(),
                token_type: token_type.clone(),
            };

            if token_type == TokenType::Operator {
                let buffer_token = Token {
                    content: buffer.iter().collect::<String>(),
                    token_type: current_buffer_type,
                };

                result.push(buffer_token);
                result.push(token);

                buffer.clear();
                current_buffer_type = TokenType::Null;
            } else if buffer.is_empty() && current_buffer_type == TokenType::Null {
                buffer.push(character);
                current_buffer_type = token_type;
            } else if token_type == current_buffer_type {
                buffer.push(character);
            } else if token_type != current_buffer_type {
                let buffer_token = Token {
                    content: buffer.iter().collect::<String>(),
                    token_type: current_buffer_type,
                };

                result.push(buffer_token);
                buffer.clear();

                buffer.push(character);

                current_buffer_type = token_type;
            }
        }

        result.push(Token {
            content: buffer.iter().collect::<String>(),
            token_type: current_buffer_type,
        });

        result = result
            .iter()
            .filter(|x| x.token_type != TokenType::Null)
            .cloned()
            .collect();

        self.clean_negatives(result)
    }

    fn clean_negatives(&self, data: Vec<Token>) -> Vec<Token> {
        let mut result = data;

        match result.len() {
            0 | 1 => result,
            2 => {
                if result[0].content == "-" && result[1].token_type == TokenType::Operand {
                    let new_token = Token {
                        content: format!("-{}", result[1].content),
                        token_type: TokenType::Operand,
                    };

                    result = vec![new_token];
                }

                result
            }
            _ => {
                let mut successful_iteration = false;
                while !successful_iteration {
                    for i in 0..result.len() - 2 {
                        if result[i].token_type == TokenType::Operator
                            && result[i + 1].content == "-"
                            && result[i + 2].token_type == TokenType::Operand
                        {
                            result[i + 2].content =
                                if result[i + 2].content.chars().nth(0).unwrap() == '-' {
                                    result[i + 2].content[1..].to_string()
                                } else {
                                    format!("-{}", result[i + 2].content)
                                };

                            result.remove(i + 1);
                            break;
                        }
                        if i == result.len() - 3 {
                            successful_iteration = true;
                        }
                    }

                    if result.len() < 3 {
                        successful_iteration = true;
                    }
                }

                if result[0].content == "-" && result[1].token_type == TokenType::Operand {
                    result[1].content = format!("-{}", result[1].content);
                    result.remove(0);
                }

                result
            }
        }
    }
}
