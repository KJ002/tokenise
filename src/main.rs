#[derive(PartialEq, Eq, Clone, Debug)]
enum TokenType {
    Operand,
    Operator,
    Other,
    Null,
}

#[derive(Clone, Debug)]
struct Token {
    content: String,
    token_type: TokenType,
}

trait Lexer {
    fn tokenise(&self) -> Result<Vec<Token>, String>;

    fn operators(&self) -> Vec<char> {
        vec!['+', '-', '*', '/', '(', ')']
    }

    fn ignores(&self) -> Vec<char> {
        vec!['.']
    }
}

fn initial_seperation(data: String) -> Result<Vec<Token>, String> {
    let mut result: Vec<Token> = vec![];

    let mut buffer: Vec<char> = vec![];
    let mut current_buffer_type: TokenType = TokenType::Null;

    for character in data.chars() {
        let is_operand =
            character.is_digit(10) || (character == '.' && !buffer.iter().any(|x| *x == '.'));
        let is_operator = data.operators().iter().any(|x| *x == character);

        if is_operand && is_operator {
            return Err(format!(
                "The character {} seems to have an abiguous type.",
                character
            ));
        }

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
    Ok(result)
}

fn negative_numbers_fix(data: Result<Vec<Token>, String>) -> Result<Vec<Token>, String> {
    let mut result = data.unwrap();

    Ok(match result.len() {
        0 => result,
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
    })
}

fn consecutive_types(data: Vec<Token>) -> Result<(), String> {
    if data.len() < 2 {
        return Ok(());
    }

    for i in 0..data.len() - 2 {
        if data[i].token_type == data[i + 1].token_type {
            return Err(format!(
                "Consecutive types at {}{}",
                data[i].content,
                data[i + 1].content
            ));
        }
    }

    Ok(())
}

fn check(data: Result<Vec<Token>, String>) -> Result<Vec<Token>, String> {
    let checks: Vec<fn(Vec<Token>) -> Result<(), String>> = vec![consecutive_types];
    let results: Vec<bool> = checks
        .iter()
        .map(|x| x(data.clone().unwrap()).unwrap() == ())
        .collect();

    // Sum the result vector
    let total: usize = results.iter().cloned().fold(0, |acc, x| acc + x as usize);

    if total < results.len() {
        return Err("The code seems to have failed a vital check.".to_string());
    }

    data
}

impl Lexer for String {
    fn tokenise(&self) -> Result<Vec<Token>, String> {
        check(negative_numbers_fix(initial_seperation(self.to_string())))
    }
}

impl Lexer for str {
    fn tokenise(&self) -> Result<Vec<Token>, String> {
        self.to_string().tokenise()
    }
}

fn main() {
    for token in "-1".tokenise().unwrap() {
        println!("{}, {:?}", token.content, token.token_type);
    }
}
