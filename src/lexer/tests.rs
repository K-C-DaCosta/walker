use super::*; 

#[test]
pub fn lexer_sanity_test_0() {
    let raw_text = "123*324";

    let mut lexer = Lexer::new();
    lexer.lex(raw_text);
    assert_eq!(
        vec![Token::Integer(123), Token::Mul, Token::Integer(324)],
        lexer.tokens
    );
}
#[test]
pub fn lexer_sanity_test_1() {
    let raw_text = "123+324";

    let mut lexer = Lexer::new();
    lexer.lex(raw_text);
    assert_eq!(
        vec![Token::Integer(123), Token::Add, Token::Integer(324)],
        lexer.tokens
    );
}
#[test]
pub fn lexer_sanity_test_2() {
    let raw_text = "123-324";

    let mut lexer = Lexer::new();
    lexer.lex(raw_text);
    assert_eq!(
        vec![Token::Integer(123), Token::Sub, Token::Integer(324)],
        lexer.tokens
    );
}
#[test]
pub fn lexer_sanity_test_3_spaces() {
    
    let raw_text = "    123    -   324   ";

    let mut lexer = Lexer::new();
    lexer.lex(raw_text);
    assert_eq!(
        vec![Token::Integer(123), Token::Sub, Token::Integer(324)],
        lexer.tokens
    );
}

#[test]
pub fn lexer_sanity_test_4() {
    
    let raw_text = "9*9*9+123+223-10/10123";

    let mut lexer = Lexer::new();
    lexer.lex(raw_text);
    
    println!("{:?}", lexer.tokens);
    
}

#[test]
pub fn lexer_sanity_test_5() {
    
    let raw_text = "1+1+";

    let mut lexer = Lexer::new();
    lexer.lex(raw_text);
    
    println!("{:?}", lexer.tokens);
    
}