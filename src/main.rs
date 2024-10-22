// Define the different types of tokens
#[derive(Debug, PartialEq, Clone)]
enum Token {
    Keyword(String),
    Identifier(String),
    Number(i32),
    StringLiteral(String),
    Equals,
    Semicolon,
    EndOfFile,
}

// Tokenize the BP source code into a list of tokens
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '=' => {
                tokens.push(Token::Equals);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '"' => {
                chars.next(); // skip starting quote
                let mut s = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        break;
                    }
                    s.push(ch);
                    chars.next();
                }
                chars.next(); // skip ending quote
                tokens.push(Token::StringLiteral(s));
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_digit(10) {
                        num.push(ch);
                    } else {
                        break;
                    }
                    chars.next();
                }
                tokens.push(Token::Number(num.parse::<i32>().unwrap()));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphabetic() {
                        ident.push(ch);
                    } else {
                        break;
                    }
                    chars.next();
                }
                match ident.as_str() {
                    "m" | "c" | "show" => tokens.push(Token::Keyword(ident)),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }
            ' ' | '\n' | '\t' => {
                chars.next(); // skip whitespace
            }
            _ => panic!("Unexpected character: {}", ch),
        }
    }

    tokens.push(Token::EndOfFile);
    tokens
}

// Define the AST
#[derive(Debug)]
enum ASTNode {
    VariableDeclaration { name: String, value: Box<ASTNode> },
    StringLiteral(String),
    NumberLiteral(i32),
    Show(String),
}

// Parse tokens into an AST
fn parse(tokens: &[Token]) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    let mut idx = 0;

    while idx < tokens.len() {
        match &tokens[idx] {
            Token::Keyword(k) => match k.as_str() {
                "m" | "c" => {
                    if let Token::Identifier(name) = &tokens[idx + 1] {
                        if let Token::Equals = tokens[idx + 2] {
                            match &tokens[idx + 3] {
                                Token::Number(num) => {
                                    ast.push(ASTNode::VariableDeclaration {
                                        name: name.clone(),
                                        value: Box::new(ASTNode::NumberLiteral(*num)),
                                    });
                                    idx += 4;
                                }
                                Token::StringLiteral(s) => {
                                    ast.push(ASTNode::VariableDeclaration {
                                        name: name.clone(),
                                        value: Box::new(ASTNode::StringLiteral(s.clone())),
                                    });
                                    idx += 4;
                                }
                                _ => panic!("Expected a number or string after `=`"),
                            }
                        }
                    }
                }
                "show" => {
                    if let Token::StringLiteral(s) = &tokens[idx + 1] {
                        ast.push(ASTNode::Show(s.clone()));
                        idx += 2;
                    }
                }
                _ => panic!("Unknown keyword: {}", k),
            },
            _ => panic!("Unexpected token: {:?}", tokens[idx]),
        }

        if let Token::Semicolon = tokens[idx] {
            idx += 1;
        }
    }

    ast
}

// Transpile AST into C code
fn transpile(ast: Vec<ASTNode>) -> String {
    let mut c_code = String::from("#include <stdio.h>\n\nint main() {\n");

    for node in ast {
        match node {
            ASTNode::VariableDeclaration { name, value } => match *value {
                ASTNode::NumberLiteral(num) => {
                    c_code.push_str(&format!("    int {} = {};\n", name, num));
                }
                ASTNode::StringLiteral(s) => {
                    c_code.push_str(&format!("    char {}[] = \"{}\";\n", name, s));
                }
                _ => {}
            },
            ASTNode::Show(s) => {
                c_code.push_str(&format!("    printf(\"{}\\n\");\n", s));
            }
            _ => {} // Here is the wildcard match that handles any other unhandled ASTNode variants
        }
    }

    c_code.push_str("    return 0;\n}");
    c_code
}

fn main() {
    let source_code = r#"
    m variable01 = 01;
    m variable02 = 02;
    c variable03 = "BP Language";
    show "The future language start here";
    "#;

    // Tokenize
    let tokens = tokenize(source_code);
    println!("Tokens: {:?}\n\n", tokens);

    // Parse into AST
    let ast = parse(&tokens);
    println!("AST: {:?}\n\n", ast);

    // Transpile into C code
    let c_code = transpile(ast);
    println!("\nGenerated C code:\n{}", c_code);
}
