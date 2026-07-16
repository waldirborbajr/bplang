use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;
use std::collections::HashMap;
use std::path::Path;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

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
                    if ch.is_ascii_digit() {
                        num.push(ch);
                    } else {
                        break;
                    }
                    chars.next();
                }
                if let Ok(n) = num.parse::<i32>() {
                    tokens.push(Token::Number(n));
                }
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
            ' ' | '\n' | '\t' | '\r' => {
                chars.next(); // skip whitespace
            }
            _ => {
                println!("Unexpected character: {:?}", ch);
                chars.next();
            }
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
    // Future: ShowVar(String), BinaryExpr, etc.
}

// Parse tokens into an AST (kept mostly as-is, improved robustness)
fn parse(tokens: &[Token]) -> Vec<ASTNode> {
    let mut ast = Vec::new();
    let mut idx = 0;
    while idx < tokens.len() {
        match &tokens[idx] {
            Token::Keyword(k) => match k.as_str() {
                "m" | "c" => {
                    if let Some(Token::Identifier(name)) = tokens.get(idx + 1) {
                        if let Some(Token::Equals) = tokens.get(idx + 2) {
                            match tokens.get(idx + 3) {
                                Some(Token::Number(num)) => {
                                    ast.push(ASTNode::VariableDeclaration {
                                        name: name.clone(),
                                        value: Box::new(ASTNode::NumberLiteral(*num)),
                                    });
                                    idx += 4;
                                    continue;
                                }
                                Some(Token::StringLiteral(s)) => {
                                    ast.push(ASTNode::VariableDeclaration {
                                        name: name.clone(),
                                        value: Box::new(ASTNode::StringLiteral(s.clone())),
                                    });
                                    idx += 4;
                                    continue;
                                }
                                _ => {}
                            }
                        }
                    }
                    println!("Invalid variable declaration at token {}", idx);
                    idx += 1;
                }
                "show" => {
                    if let Some(Token::StringLiteral(s)) = tokens.get(idx + 1) {
                        ast.push(ASTNode::Show(s.clone()));
                        idx += 2;
                        continue;
                    }
                    println!("Expected string after 'show'");
                    idx += 1;
                }
                _ => idx += 1,
            },
            Token::Semicolon | Token::EndOfFile => break,
            _ => idx += 1,
        }
    }
    ast
}

// Execute an AST directly. Shared by file mode and the REPL so both run
// BPLang the same way — no C, no external compiler.
//
// `interactive`: when true (REPL), variable declarations echo `name = value`
// like a shell would. In file mode this is off, matching the previous
// behavior where declarations produced no output unless `show`n.
fn interpret(ast: Vec<ASTNode>, variables: &mut HashMap<String, String>, interactive: bool) {
    for node in ast {
        match node {
            ASTNode::Show(s) => println!("{}", s),
            ASTNode::VariableDeclaration { name, value } => {
                let val_str = match *value {
                    ASTNode::StringLiteral(s) => {
                        variables.insert(name.clone(), s.clone());
                        format!("\"{}\"", s)
                    }
                    ASTNode::NumberLiteral(n) => {
                        variables.insert(name.clone(), n.to_string());
                        n.to_string()
                    }
                    _ => "unknown".to_string(),
                };
                if interactive {
                    println!("{} = {}", name, val_str);
                }
            }
            _ => {}
        }
    }
}

fn run_file() -> io::Result<()> {
    let bp_file_path = "main.bp";
    let mut source_code = String::new();
    File::open(bp_file_path)?.read_to_string(&mut source_code)?;

    let tokens = tokenize(&source_code);
    let ast = parse(&tokens);

    let mut variables: HashMap<String, String> = HashMap::new();
    interpret(ast, &mut variables, false);
    Ok(())
}

fn run_prompt() {
    println!("BP Interactive Prompt v{} (type 'exit' or Ctrl+C to quit)", env!("CARGO_PKG_VERSION"));
    let mut variables: HashMap<String, String> = HashMap::new();
    let mut editor = DefaultEditor::new().expect("failed to initialize line editor");

    loop {
        match editor.readline("> ") {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                let _ = editor.add_history_entry(input);

                if input == "exit" || input == "quit" {
                    break;
                }

        let tokens = tokenize(input);
        let ast = parse(&tokens);

        for node in ast {
            match node {
                ASTNode::Show(s) => println!("{}", s),
                ASTNode::VariableDeclaration { name, value } => {
                    let val_str = match *value {
                        ASTNode::StringLiteral(s) => {
                            variables.insert(name.clone(), s.clone());
                            format!("\"{}\"", s)
                        }
                        ASTNode::NumberLiteral(n) => {
                            variables.insert(name.clone(), n.to_string());
                            n.to_string()
                        }
                        _ => "unknown".to_string(),
                    };
                    println!("{} = {}", name, val_str);
                }
                _ => {}
            }
        }
    }
}

fn main() -> io::Result<()> {
    if Path::new("main.bp").exists() {
        run_file()
    } else {
        run_prompt();
        Ok(())
    }
}