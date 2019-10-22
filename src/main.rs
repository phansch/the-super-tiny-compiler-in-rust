fn main() {
    println!("Hello, world!");
    tokenizer(String::from("foo"));
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Paren(char),
    Str(String),
    Name(String),
}

fn tokenizer(input: String) -> Vec<Token> {
    let mut current = 0;

    let mut tokens = vec![];

    while current < input.len() {
        if let Some(mut current_char) = input.chars().nth(current) {
            if current_char == '(' {
                tokens.push(Token::Paren('('));
                current += 1;
                continue;
            }
            if current_char == ')' {
                tokens.push(Token::Paren(')'));
                current += 1;
                continue;
            }

            if current_char.is_whitespace() {
                current += 1;
                continue;
            }
            if current_char.is_numeric() {
                let mut value = String::new();

                value.push(current_char);

                current += 1;
                continue;
            }

            if current_char == '"' {
                let mut value = String::new();
                // Skip starting double quote
                current += 1;
                current_char = input.chars().nth(current).unwrap();
                while current_char != '"' {
                    value.push(current_char);
                    current += 1;
                    current_char = input.chars().nth(current).unwrap();
                }
                // Skip the closing double quote
                current += 1;
                tokens.push(Token::Str(value));
                continue;
            }

            if current_char.is_alphabetic() {
                let mut value = String::new();
                while current_char.is_alphabetic() {
                    value.push(current_char);
                    current += 1;
                    current_char = input.chars().nth(current).unwrap();
                }
                tokens.push(Token::Name(value));
                continue;
            }

            // If nothing matched, panic!
            panic!("Unhandled character: {}", current_char);
        }
    }
    tokens
}

#[test]
fn tokenizer_test() {
    assert_eq!(
        vec![Token::Paren('('), Token::Name("foo".to_string()), Token::Str("abc".to_string()), Token::Paren(')')],
        tokenizer(String::from(r#"(foo "abc")"#))
    );
}
