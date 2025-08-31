use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process::exit;

pub fn slice_char_to_str(slice: &[char]) -> String{
    let mut word = String::new();
    for ch in slice{
	word.push(*ch);
    }
    word
} // Had to write this $hit because the fuckin' "C killer" is not able to convert slice char to str in it's normal version (-_-)

fn main() {
    let ast: Vec<String> = Vec::<String>::new();
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
    #[derive(Debug)]
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
	ELSE
    }
    struct Token{
	value: String,
	ttype: TokenType
    }
    
    let mut tokens: Vec<Token> = Vec::<Token>::new();
    let mut i = 0;
    while i < chars.len(){
	match chars[i]{
	    ' ' => {
		println!("WhiteSPACE");
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
	    }
	    _ => {
		if chars[i].is_alphanumeric(){
		    let start = i;
		    while i < chars.len() && chars[i].is_alphanumeric(){i+=1};
		    let word = slice_char_to_str(&chars[start..i]);
		    println!("{word}");// TODO: Figure out why this went wrong:(((
		    match word.as_str(){
			"print" => {
			    tokens.push(Token{value: word, ttype: TokenType::PRINT});
			    i+=1
			},
			"if" => {
			    tokens.push(Token{value: word, ttype: TokenType::IF});
			    i+=1
			},
			"then" => {
			    tokens.push(Token{value: word, ttype: TokenType::THEN});
			    i+=1
			},
			"else" => {
			    tokens.push(Token{value: word, ttype: TokenType::ELSE});
			    i+=1
			},
			_ => i+=1,
		    }
		}
		i+=1;
	    },
	}
    }
    for token in &tokens{
	println!("<{}>:<{:?}>", token.value, token.ttype);
    }
}

fn parser(ast: Vec<String>){
    println!("{ast:?}");
}
