use ethers_core::abi::{Token};
use ethers_core::types::I256;

pub fn format_tokens(tokens: &Vec<Token>) -> String {
    match tokens.len() {
        0 => "".to_string(),
        1 => format_token(&tokens[0]),
        _ => format!("[{}]", format_arr(tokens)),
    }
}

pub fn format_token(token: &Token) -> String {
    match token {
        Token::Bool(b) => format!("{}", b),
        Token::String(s) => format!("{}", s),
        Token::Address(a) => format!("0x{:x}", a),
        Token::Bytes(bytes)=> format!("0x{}", hex::encode(bytes)),
        Token::FixedBytes(bytes) => format!("0x{}", hex::encode(bytes)),
        Token::Uint(i)=> i.to_string(),
        Token::Int(i) => format!("{}", I256::from_raw(*i)),
        Token::Tuple(arr) => format!("({})", format_arr(arr)),
        Token::Array(arr) => format!("[{}]", format_arr(arr)),
        Token::FixedArray(arr) => format!("[{}]", format_arr(arr))
    }
}

fn format_arr(arr: &Vec<Token>) -> String {
    arr
        .iter()
        .map(format_token)
        .collect::<Vec<String>>()
        .join(",")
}