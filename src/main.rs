use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;

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
            _ => panic!("Unexpected character: {:?}", ch),
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
            Token::Keyword(k) => {
                match k.as_str() {
                    "m" | "c" => {
                        if idx + 1 < tokens.len() {
                            if let Token::Identifier(name) = &tokens[idx + 1] {
                                if idx + 2 < tokens.len() {
                                    if let Token::Equals = &tokens[idx + 2] {
                                        if idx + 3 < tokens.len() {
                                            match &tokens[idx + 3] {
                                                Token::Number(num) => {
                                                    ast.push(ASTNode::VariableDeclaration {
                                                        name: name.clone(),
                                                        value: Box::new(ASTNode::NumberLiteral(
                                                            *num,
                                                        )),
                                                    });
                                                    idx += 4; // Move past the variable declaration
                                                }
                                                Token::StringLiteral(s) => {
                                                    ast.push(ASTNode::VariableDeclaration {
                                                        name: name.clone(),
                                                        value: Box::new(ASTNode::StringLiteral(
                                                            s.clone(),
                                                        )),
                                                    });
                                                    idx += 4; // Move past the variable declaration
                                                }
                                                _ => {
                                                    println!("Unexpected value after `=` at token index: {}", idx + 3);
                                                    idx += 1; // Move to the next token
                                                }
                                            }
                                        } else {
                                            println!("Expected a value after `=` but reached end of tokens.");
                                            idx += 1;
                                        }
                                    } else {
                                        println!(
                                            "Expected `=` after identifier `{}` at token index: {}",
                                            name,
                                            idx + 1
                                        );
                                        idx += 1; // Move to the next token
                                    }
                                } else {
                                    println!(
                                        "Expected `=` after identifier but reached end of tokens."
                                    );
                                    idx += 1;
                                }
                            } else {
                                println!(
                                    "Expected identifier after keyword `{}` at token index: {}",
                                    k,
                                    idx + 1
                                );
                                idx += 1;
                            }
                        } else {
                            println!("Expected identifier but reached end of tokens.");
                            idx += 1;
                        }
                    }
                    "show" => {
                        if idx + 1 < tokens.len() {
                            if let Token::StringLiteral(s) = &tokens[idx + 1] {
                                ast.push(ASTNode::Show(s.clone()));
                                idx += 2; // Move past the `show` statement
                            } else {
                                println!(
                                    "Expected string literal after `show` at token index: {}",
                                    idx + 1
                                );
                                idx += 1;
                            }
                        } else {
                            println!("Expected string literal but reached end of tokens.");
                            idx += 1;
                        }
                    }
                    _ => {
                        println!("Unknown keyword: {} at token index: {}", k, idx);
                        idx += 1; // Move to the next token
                    }
                }
            }
            Token::Semicolon => {
                // Skip semicolons, move to the next token
                idx += 1;
            }
            Token::EndOfFile => {
                break; // Exit the loop when end of file token is reached
            }
            _ => {
                println!("Unexpected token: {:?} at index: {}", tokens[idx], idx);
                idx += 1; // Move to the next token
            }
        }
    }

    ast
}

// Transpile AST into C code and write it to a file
fn transpile_and_write_c(ast: Vec<ASTNode>, output_file: &str) -> io::Result<()> {
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
            _ => {} // Handles other unhandled ASTNode variants
        }
    }

    c_code.push_str("    return 0;\n}");

    // Write the C code to the output file
    let mut file = File::create(output_file)?;
    file.write_all(c_code.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    // Step 1: Read BP source code from the `main.bp` file
    let bp_file_path = "main.bp";
    let mut bp_file = File::open(bp_file_path)?;
    let mut source_code = String::new();
    bp_file.read_to_string(&mut source_code)?;

    // Step 2: Tokenize the BP source code
    let tokens = tokenize(&source_code);
    //println!("Tokens: {:?}", tokens);

    // Step 3: Parse tokens into AST
    let ast = parse(&tokens);
    //println!("AST: {:?}", ast);

    // Step 4: Transpile AST to C code and write to `main.c`
    transpile_and_write_c(ast, "main.c")?;
    println!("C code has been generated and written to main.c");

    // Step 5: Compile the generated C code using `gcc`
    let output = Command::new("gcc")
        .arg("main.c")
        .arg("-o")
        .arg("main") // Output binary file `main`
        .output()?;

    // Check if the compilation was successful
    if !output.status.success() {
        println!("Compilation failed:");
        io::stderr().write_all(&output.stderr)?;
        return Err(io::Error::new(io::ErrorKind::Other, "C compilation failed"));
    }

    println!("Compilation successful, running the program...");

    // Step 6: Execute the compiled binary
    let execution_output = Command::new("./main").output()?;

    // Print the output of the program
    println!("Program output:");
    io::stdout().write_all(&execution_output.stdout)?;

    Ok(())
}
