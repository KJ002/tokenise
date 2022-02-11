mod lexer;
mod modles;
use crate::lexer::Lexer;
use crate::modles::Token;

trait Tokenise {
    fn tokenise(&self) -> Vec<Token>;
}

impl Tokenise for str {
    fn tokenise(&self) -> Vec<Token> {
        Lexer::new(self.to_string()).tokenise()
    }
}

fn main() {
    for token in "1".tokenise() {
        println!("{}, {:?}", token.content, token.token_type);
    }
}
