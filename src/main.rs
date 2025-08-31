use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process::exit;

// pub fn slice_char_to_str(slice: &[char]) -> String{
//     let mut word = String::new();
//     for ch in slice{
// 	word.push(*ch);
//     }
//     word
// } // Had to write this $hit because the fuckin' "C killer" is not able to convert slice char to str in it's normal version (-_-)
#[derive(Debug)]
#[derive(PartialEq)]
enum TokenType{
    PLUS,
    MINUS,
    MULTI,
    DEVIDE,
    EQUAL,
    NUMBER,
    PRINT,
    IF,
    THEN,
    INPUT,
    GOTO,
    FOR,
    TO,
    NEXT,
    GOSUB,
    RETURN,
    END,
    ELSE,
    ID,
    STRING,
    BIGGERTHAN,
    LESSTHAN,
    TRUE,
    FALSE,
    EOL
}
#[derive(Debug)]
struct Token{
    value: String,
    ttype: TokenType
}
enum Ast<T> {
    Node{
	value: T,
	left: Option<Box<Ast<T>>>,
	right: Option<Box<Ast<T>>>
    },
    Leaf(T),
    Empty
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1{
	eprintln!("Usage: VBL [filepath]");
	exit(1);
    }
    let mut file = File::open(&args[1])
	.expect("Cant't Open File :(  ERROR: ");
    let mut content = String::new();
    file.read_to_string(&mut content)
	.expect("Cant't Read File :(  ERROR: ");
    let chars: Vec<char> = content.chars().collect();
    parser(tokenizer(chars));
}
fn tokenizer(chars: Vec<char>) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::<Token>::new();
    let mut i = 0;
    while i < chars.len(){
	match chars[i]{
	    ' ' => {
		i+=1
	    },
	    '+' => {
		tokens.push(Token{value: chars[i].to_string(), ttype: TokenType::PLUS});
		i+=1
	    },
	    '*' => {
		tokens.push(Token{value: chars[i].to_string(), ttype: TokenType::MULTI});
		i+=1
	    },
	    '/' => {
		tokens.push(Token{value: chars[i].to_string(), ttype: TokenType::DEVIDE});
		i+=1
	    },
	    '-' => {
		tokens.push(Token{value: chars[i].to_string(), ttype: TokenType::MINUS});
		i+=1
	    },
	    '>' => {
		tokens.push(Token{value: chars[i].to_string(), ttype: TokenType::BIGGERTHAN});
		i+=1
	    },
	    '<' => {
		tokens.push(Token{value: chars[i].to_string(), ttype: TokenType::LESSTHAN});
		i+=1
	    },
	    '=' => {
		tokens.push(Token{value: chars[i].to_string(), ttype: TokenType::EQUAL});
		i+=1
	    },
	    '"' => {
		i+=1;
		let start = i;
		while i < chars.len() && chars[i] != '"'{
		    i+=1;
		}
		let text: String = chars[start..i]
		    .iter()
		    .collect();
		i+=1;
		tokens.push(Token{value: text, ttype: TokenType::STRING});
	    },
	    _ => {
		if chars[i].is_numeric(){
		    let start = i;	 
		    while i < chars.len() && chars[i].is_numeric() {
			i += 1;
		    } 
		    let num: String = chars[start..i]
			.iter()
			.collect();	    
		    tokens.push(Token {value: num.clone(), ttype: TokenType::NUMBER});
		}
		if chars[i].is_alphanumeric(){
		    let word: String = chars[i..]
			.iter()
			.take_while(|&&c| c.is_alphanumeric())
			.collect();
		    i += word.len();
		    match word.as_str(){
			"print" => {
			    tokens.push(Token{value: word, ttype: TokenType::PRINT});
			},
			"if" => {
			    tokens.push(Token{value: word, ttype: TokenType::IF});
			},
			"then" => {
			    tokens.push(Token{value: word, ttype: TokenType::THEN});
			},
			"else" => {
			    tokens.push(Token{value: word, ttype: TokenType::ELSE});
			},
			"true" => {
			    tokens.push(Token{value: word, ttype: TokenType::TRUE});
			},
			_ => {
			    tokens.push(Token{value: word, ttype: TokenType::ID})
			},
		    }
		}
	    },
	}
	i+=1
    }
    tokens
}
fn parser(tokens: Vec<Token>){
    let mut i = 0;
    while i < tokens.len(){
	let mut token = &tokens[i];
	println!("{:#?}", token);
	match token.ttype{
	    TokenType::PLUS => {
		println!("<PLUS> RIGHT<{:?}> LEFT<{:?}>", tokens[i+1], tokens[i-1]);
	    },
	    TokenType::EQUAL => {
		println!("<EQUAL> RIGHT<{:?}> LEFT<{:?}>", tokens[i+1], tokens[i-1]);
	    },
	    _ =>{}
	}
	i+=1
    }
}
