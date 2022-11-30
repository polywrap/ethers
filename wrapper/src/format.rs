use ethabi::token::Tokenizer;

use ethers_core::abi::{Param, ParamType, Token};

pub fn params_to_types(params: &Vec<Param>) -> Vec<ParamType> {
    params.clone().into_iter().map(|i| i.kind).collect()
}

pub fn tokenize_values(values: &Vec<String>, kinds: &Vec<ParamType>) -> Vec<Token> {
    values
        .iter()
        .zip(kinds.iter())
        .map(|(arg, kind)| ethabi::token::LenientTokenizer::tokenize(kind, arg).unwrap())
        .collect()
}

// custom format because serde_json::to_string doesn't work on Vec<Token>
pub fn format_token(token: &Token) -> String {
    match token {
        Token::Bool(b) => format!("{}", b),
        Token::String(ref s) => format!("{}", s),
        Token::Address(ref a) => format!("0x{:x}", a),
        Token::Bytes(ref bytes) | Token::FixedBytes(ref bytes) => {
            format!("{}", hex::encode(&bytes))
        }
        Token::Uint(ref i) | Token::Int(ref i) => format!("{}", i),
        Token::Array(ref arr) | Token::FixedArray(ref arr) => {
            let s = arr
                .iter()
                .map(|ref t| format!("{}", t))
                .collect::<Vec<String>>()
                .join(",");
            format!("[{}]", s)
        }
        Token::Tuple(ref s) => {
            let s = s
                .iter()
                .map(|ref t| format!("{}", t))
                .collect::<Vec<String>>()
                .join(",");
            format!("({})", s)
        }
    }
}

pub fn format_tokens(tokens: &Vec<Token>) -> String {
    match tokens.len() {
        0 => "".to_string(),
        1 => format_token(&tokens[0]),
        _ => format!(
            "[{}]",
            tokens
                .iter()
                .map(format_token)
                .collect::<Vec<String>>()
                .join(",")
        ),
    }
}