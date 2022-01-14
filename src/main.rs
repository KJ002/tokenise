#[derive(PartialEq, Eq, Clone, Debug)]
enum TokenType {
    Operand,
    Operator,
    Other,
    Null,
}

#[allow(dead_code)]
#[derive(Clone)]
struct Token {
    content: String,
    token_type: TokenType,
}

trait Lexer {
    fn tokenise(&self) -> Result<Vec<Token>, String>;

    fn operators(&self) -> Vec<char> {
        vec!['+', '-', '*', '/', '(', ')']
    }
}

fn initial_seperation(data: String) -> Result<Vec<Token>, String>{
    let mut result: Vec<Token> = vec![];

    let mut buffer: Vec<char> = vec![];
    let mut current_buffer_type: TokenType = TokenType::Null;

    for character in data.chars() {
        let is_operand = character.is_digit(10);
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
            token_type: token_type.clone()
        };

        if token_type == TokenType::Operator {
            let buffer_token = Token {
                content: buffer.iter().collect::<String>(),
                token_type: current_buffer_type
            };

            result.push(buffer_token);
            result.push(token);

            buffer.clear();
            current_buffer_type = TokenType::Null;
        } else if buffer.is_empty() && current_buffer_type == TokenType::Null{
            buffer.push(character);
            current_buffer_type = token_type;
        } else if token_type == current_buffer_type {
            buffer.push(character);
        } else if token_type != current_buffer_type {
            let buffer_token = Token {
                content: buffer.iter().collect::<String>(),
                token_type: current_buffer_type
            };

            result.push(buffer_token);
            buffer.clear();

            buffer.push(character);

            current_buffer_type = token_type;
        }
    }

    result.push( Token {
        content: buffer.iter().collect::<String>(),
        token_type: current_buffer_type
    });

    result = result.iter().filter(|x| x.token_type != TokenType::Null).cloned().collect();
    Ok(result)
}

fn negative_numbers_fix(data: Result<Vec<Token>, String>) -> Result<Vec<Token>, String> {
    let mut result = data.unwrap();

    if result[0].content == "-" && result[1].token_type == TokenType::Operand {
        result[1].content = format!("-{}", result[1].content);
        result.remove(0);
    }

    let mut successful_iteration = false;
    while !successful_iteration{
        for i in 0..result.len()-3{
            if result[i].token_type == TokenType::Operator && result[i+1].content == "-" && result[i+2].token_type == TokenType::Operand {
                result[i+2].content = format!("-{}", result[i+2].content);
                result.remove(i+1);
                break;
            }

            else if i == result.len()-4{
                successful_iteration = true;
            }
        }
    }

    Ok(result)
}

impl Lexer for String {
    fn tokenise(&self) -> Result<Vec<Token>, String> {
        negative_numbers_fix(initial_seperation(self.to_string()))
    }
}

fn main() {
    for token in "-11+-1*8xy".to_string().tokenise().unwrap() {
        println!("{}, {:?}", token.content, token.token_type);
    }
}
