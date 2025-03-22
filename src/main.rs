use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1])
	.expect("Cant't Open File :(  ERROR: ");
    let mut content = String::new();
    file.read_to_string(&mut content)
	.expect("Cant't Read File :(  ERROR: ");
    let chars: Vec<char> = content.chars().collect();
    /*let mut tokens = HashMap::new();
    tokens.insert(
	"equal".to_string(),
	"=".to_string()
    );
    tokens.insert(
	"hello".to_string(),
	"Hello to YOUU".to_string()
    );
    let mut words = Vec::<String>::new();*/
    for ch in chars{
	match ch {
            '+' => /*Some(Token::Add)*/ println!("add"),
	    '-' => /*Some(Token::Minus)*/ println!("minus"),
	    '*' => /*Some(Token::Mul)*/ println!("multi"),
	    '/' => /*Some(Token::Div)*/ println!("devide"),
	    '=' => /*Some(Token::Div)*/ println!("equals"),
	    ch if ch.is_numeric() => println!("n"),
	    ch if ch.is_alphanumeric() => println!("an"),
	    ch if ch.is_whitespace() => /*Some(Token::Whitespace)*/ println!("whitespace"),
	    _ => println!("Not Compatable"),
        };


	//let mut word = String::new();
	//if ch.is_whitespace(){
	//    if words.len() > 0{
	//	words.push(word.clone())
	//    }else{
	//	println!("Whitespace");
	//    }
	//}else{
	//    word.push(ch);
	//}
    }
   // for word in words{
//	println!("{}", word);
//	match tokens.get(&word.to_string()) {
//	    Some(msg) => println!("{}", msg),
//	    None => println!("Not compatable {}", word)
//	}
  //  }
}
