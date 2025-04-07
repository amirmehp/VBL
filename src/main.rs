use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])
	.expect("Cant't Open File :(  ERROR: ");
    let mut content = String::new();
    file.read_to_string(&mut content)
	.expect("Cant't Read File :(  ERROR: ");
    let chars: Vec<char> = content.chars().collect();
   
    let mut tokens: Vec<String> = Vec::<String>::new();
    let mut token = String::new();

    for window in chars.as_slice().windows(2){
		let ch = window[0] as char;
		let next_char = window[1] as char;
		match ch {
			'+' => {
				println!("<PLUS>");
				tokens.push(token.clone());
				token = "".to_string();
			},
			'-' => {
				println!("<MINUS>");
				tokens.push(token.clone());
				token = "".to_string();
			},
			'*' => {
				println!("<MULTI>");
				tokens.push(token.clone());
				token = "".to_string();
			},
			'/' => {
				println!("<DEVIDE>");
				tokens.push(token.clone());
				token = "".to_string();
			},
			'=' => {
				println!("<EQUAL>");
				tokens.push(token.clone());
				token = "".to_string();
			},
			ch if ch.is_numeric() => {
				token.push_str(&ch.to_string());
				if next_char.is_whitespace(){
					println!("<NUMBER> {}", token);
					tokens.push(token.clone());
					token = "".to_string();
				}
			},
			ch if ch.is_alphanumeric() => {
				token.push_str(&ch.to_string());
				if next_char.is_whitespace(){
					//println!("<id> {}", token);
					tokens.push(token.clone());
					token = "".to_string();
				}
			},
			ch if ch.is_whitespace() => {
				//println!("<space>");
			}
			_ => println!("ERR: Uknown Character {}", ch),
		};
    }
	let mut print = false;
	let mut if_tk = false;
	for token in tokens{
		match token.as_str(){
			"print" => {
				println!("<PRINT>");
				print = true;
			},
			"if" => {
				println!("<IF>");
				if_tk = true;
				print = false;
			},
			"then" => println!("<THEN>"),
			"input" => println!("<INPUT>"),
			"goto" => println!("<GOTO>"),
			"for" => println!("<FOR>"),
			"to" => println!("<TO>"),
			"next" => println!("<NEXT>"),
			"gosub" => println!("<GOSUB>"),
			"return" => println!("<RETURN>"),
			"end" => println!("<END>"),
			"else" => println!("<ELSE>"),
			_ => {
				if print == true{
					println!("PRINT: {}", token);
				}else if if_tk == true{
					println!("IF: {}", token);
				}
				else{
					println!("ERR: Token `{}` can't be parsed!", token);
				}
			},
		}
	}
}
