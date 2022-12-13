use ethers_core::abi::Token;
use ethers_core::types::{I256, U256};

// format tokens to json
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
        Token::String(s) => s.to_string(),
        Token::Address(a) => format!("0x{:x}", a),
        Token::Bytes(bytes)=> format!("0x{}", hex::encode(bytes)),
        Token::FixedBytes(bytes) => format!("0x{}", hex::encode(bytes)),
        Token::Uint(i)=> i.to_string(),
        Token::Int(i) => I256::from_raw(*i).to_string(),
        Token::Tuple(arr) => format!("[{}]", format_arr(arr)),
        Token::Array(arr) => format!("[{}]", format_arr(arr)),
        Token::FixedArray(arr) => format!("[{}]", format_arr(arr))
    }
}

fn format_token_in_arr(token: &Token) -> String {
    match token {
        Token::String(s) => format!("\"{}\"", s),
        Token::Address(a) => format!("\"0x{:x}\"", a),
        Token::Bytes(bytes)=> format!("\"0x{}\"", hex::encode(bytes)),
        Token::FixedBytes(bytes) => format!("\"0x{}\"", hex::encode(bytes)),
        Token::Uint(i)=> {
            if i.bits() > 64 { format!("\"{}\"", i) }
            else { i.to_string() }
        },
        Token::Int(i) => {
            let signed_i = I256::from_raw(*i);
            if signed_i.bits() > 64  { format!("\"{}\"", signed_i) }
            else { signed_i.to_string() }
        },
        _ => format_token(token)
    }
}

fn format_arr(arr: &Vec<Token>) -> String {
    arr
        .iter()
        .map(format_token_in_arr)
        .collect::<Vec<String>>()
        .join(",")
}