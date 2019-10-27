use crate::lexer::Token;

#[derive(Debug, Eq, PartialEq)]
pub enum AstNode {
    NumberLiteral(String),
    StringLiteral(String),
    CallExpression(CallExpr),
}

#[derive(Debug, Eq, PartialEq)]
pub struct CallExpr {
    pub name: String,
    pub params: Vec<AstNode>
}

pub fn parser(tokens: Vec<Token>) -> Vec<AstNode> {
    let mut current = 0;
    let size = tokens.len();

    let mut ast = vec![];

    while current < size {
        let (token, curr) = token_walk(current, &tokens);
        current = curr;
        ast.push(token);
    }

    ast
}

// Returns single `AstNode`'s
fn token_walk(current: usize, tokens: &[Token]) -> (AstNode, usize) {
    let mut current = current;
    let mut token = &tokens[current];

    let node = match token {
        Token::Numeric(v) => {
            current += 1;
            return (AstNode::NumberLiteral(v.to_string()), current)
        },
        Token::Str(v) => {
            current += 1;
            return (AstNode::StringLiteral(v.to_string()), current)
        },
        Token::Paren(v) => {
            match v {
                '(' => {
                    // Skip paren, go to next Token, which is required to be Token::Name
                    current += 1;
                    token = &tokens[current];

                    if let Token::Name(ref token_name) = token {
                        let mut params: Vec<AstNode> = vec![];

                        // now advance to the next token
                        current += 1;
                        token = &tokens[current];

                        // Loop until we encounter `Token::Paren(')')`
                        loop {
                            if &Token::Paren(')') == token {
                                break
                            }
                            params.push(token_walk(current, tokens).0);
                            current += 1;
                            token = &tokens[current];
                        }

                        current += 1;

                        AstNode::CallExpression(CallExpr {
                            name: token_name.to_string(),
                            params
                        })
                    } else {
                        panic!("Opening paren must be followed by a name");
                    }
                },
                ')' => {
                    panic!("TODO");
                },
                character => {
                    panic!("Unsupported Token::Paren value: '{}'", character);
                }
            }
        },
        Token::Name(_) => {
            panic!("Function names are only allowed in call expressions");
        }
    };
    (node, current)
}

#[test]
fn parser_test() {
    let expected: Vec<AstNode> = vec![];
    assert_eq!(
        expected,
        parser(vec![])
    );
    let expected = vec![AstNode::NumberLiteral("1".to_string())];
    assert_eq!(
        expected,
        parser(vec![Token::Numeric("1".to_string())])
    );

    let expected = vec![AstNode::StringLiteral("A".to_string())];
    assert_eq!(
        expected,
        parser(vec![Token::Str("A".to_string())])
    );

    let expected = vec![AstNode::CallExpression(CallExpr { name: "A".to_string(), params: vec![] })];
    assert_eq!(
        expected,
        parser(vec![Token::Paren('('), Token::Name("A".to_string()), Token::Paren(')')])
    );
}
