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
   
    let mut words: Vec<String> = Vec::<String>::new();
    let mut word = String::new();    
    for window in chars.as_slice().windows(2){
		let ch = window[0] as char;
		let next_char = window[1] as char;
		match ch {
			'+' => println!("add"),
			'-' => println!("minus"),
			'*' => println!("multi"),
			'/' => println!("devide"),
			'=' => println!("equals"),
			ch if ch.is_numeric() => {
				if next_char.is_whitespace(){
					println!("<num>");
					word.push_str(&ch.to_string());
				}
			},
			ch if ch.is_alphanumeric() => {
				if next_char.is_whitespace(){
					println!("<id>");
					word.push_str(&ch.to_string());
				}
			},
			ch if ch.is_whitespace() => println!("<space>"),
			_ => println!("Not Compatable"),
		};

		for item in &words{
			println!("WORDS: {}", item);
		}
    }
}
