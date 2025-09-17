// pub fn slice_char_to_str(slice: &[char]) -> String{
//     let mut word = String::new();
//     for ch in slice{
// 	word.push(*ch);
//     }
//     word
// } // Had to write this $hit because the fuckin' "C killer" is not able to convert slice char to str in it's normal version (-_-)
#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType{
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
    EOL,
    LINENUMBER,
    LET
}
#[derive(Debug)]
pub struct Token{
    pub value: String,
    pub ttype: TokenType
}
#[derive(Debug)]
pub struct Tokenizer {
    pub input: Vec<char>,
    pub position: usize,
    pub current_char: Option<char>,
}
impl Tokenizer{
    pub fn new(chars: Vec<char>) -> Self{
	let mut tokenizer = Tokenizer{
	    input: chars,         
    	    position: 0,          
	    current_char: None
	};
	tokenizer.advance();
	tokenizer
    }
    fn advance(&mut self) -> Option<char>{
	if self.position >= self.input.len(){
	    self.current_char = None;
	    return None; 
	}
	self.current_char = Some(self.input[self.position]);
        self.position += 1;
        self.current_char
    }
    fn peek(&self) -> Option<char>{
	if self.position < self.input.len(){
	    Some(self.input[self.position])
	}else{
	    None
	}
    }
    fn skip_whitespace(&mut self) -> (){
	while let Some(c) = self.current_char {
	    if !c.is_whitespace() {
                break;
            }
            self.advance();
	}
    }
    fn read_number(&mut self) -> f64 {
	let mut number_str = String::new();
	let mut has_decimal = false;

	while let Some(c) = self.current_char {
	    if c.is_numeric(){
		number_str.push(c);
		self.advance();
	    }else if c == '.' && !has_decimal {
		number_str.push(c);
		has_decimal = true;
		self.advance();
	    }else{
		break;
	    }
	}
	number_str.parse().unwrap_or(0.0)
    }
    fn read_identifier(&mut self) -> String {
	let mut identifier = String::new();
	while let Some(c) = self.current_char {
	    if c.is_alphanumeric() || c == '_'{
		identifier.push(c);
		self.advance();
	    }
	    else{
		break;
	    }
	}
	identifier
    }
    pub fn tokenize(&mut self) -> Vec<Token>{
	let mut tokens = Vec::new();
	while let Some(c) = self.current_char {
	    self.skip_whitespace();
	    match c {
		c if c.is_digit(10) && self.peek() == Some(' ') => {
                    let line_num: u32 = self.read_identifier().parse().unwrap_or(0);
		    println!("LINE NUMBER: {}", line_num);
                    tokens.push(Token{value: line_num.to_string(), ttype: TokenType::LINENUMBER});
		}
		c if c.is_digit(10) => {
		    let number = self.read_number();
		    println!("NUMBER: {}", number);
		    tokens.push(Token{value: number.to_string(), ttype: TokenType::NUMBER});
		},
		c if c.is_alphabetic() => {
                    let identifier = self.read_identifier().to_uppercase();
                    let token = match identifier.as_str() {
			"PRINT" => Token{value: identifier, ttype: TokenType::PRINT},
			"LET" => Token{value: identifier, ttype: TokenType::LET},
			"INPUT" => Token{value: identifier, ttype: TokenType::INPUT},
			"GOTO" => Token{value: identifier, ttype: TokenType::GOTO},
			"GOSUB" => Token{value: identifier, ttype: TokenType::GOSUB},
			"RETURN" => Token{value: identifier, ttype: TokenType::RETURN},
			"IF" => Token{value: identifier, ttype: TokenType::IF},
			"THEN" => Token{value: identifier, ttype: TokenType::THEN},
			"ELSE" => Token{value: identifier, ttype: TokenType::ELSE},
			"END" => Token{value: identifier, ttype: TokenType::END},
			_ => Token{value: identifier, ttype: TokenType::ID},
                    };
                    tokens.push(token);
		},
		_ => {
		   //println!("Uknown char: {}", c);
		   // tokens.push(Token{value: line_num.to_string(), ttype: TokenType::ID});
		    self.advance();
		}
	    }
	}
	tokens
    } 
}
