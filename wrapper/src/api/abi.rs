use ethers_core::{abi::{Param, Token, encode, HumanReadableParser, token::LenientTokenizer, token::Tokenizer, Function, Abi}, types::{Bytes}};
use crate::polywrap_provider::error::WrapperError;

pub fn encode_params(types: Vec<String>, values: Vec<String>) -> Vec<u8> {
    let tokens: Vec<Token> = values
        .iter()
        .zip(types.iter())
        .map(|(arg, t)| {
            let kind = HumanReadableParser::parse_type(&t).unwrap();
            LenientTokenizer::tokenize(&kind, arg).unwrap()
        })
        .collect();
    let bytes = encode(&tokens);
    bytes
}

pub fn encode_function(method: &str, args: &Vec<String>) -> Result<(Function, Bytes), WrapperError> {
    let function: Function = parse_method(method)?;
    let tokens: Vec<Token> = tokenize_values(&args, &function.inputs);
    let bytes: Bytes = function.encode_input(&tokens).map(Into::into)?;
    Ok((function, bytes))
}

pub fn decode_function(method: &str, data: Vec<u8>) -> Vec<Token> {
    let function: Function = parse_method(method).unwrap();
    let sig = function.short_signature();
    let mut has_sig = false;

    if data[0..4] == sig {
        has_sig = true;
    }

    let arg_bytes: &[u8] = match has_sig {
        true => &data[4..],
        false => &data[0..]
    };

    function.decode_input(arg_bytes).unwrap()
}

pub fn tokenize_values(values: &Vec<String>, params: &Vec<Param>) -> Vec<Token> {
    params
        .iter()
        .zip(values.iter())
        .map(|(param, arg)| LenientTokenizer::tokenize(&param.kind, arg).unwrap())
        .collect()
}

fn parse_method(method: &str) -> Result<Function, WrapperError> {
    let parse_result = HumanReadableParser::parse_function(method).map_err(|e| {
        WrapperError::LexerError(format!("{:?}", e))
    });
    if parse_result.is_ok() {
        parse_result
    } else {
        let json_parse: Result<Abi, serde_json::Error>;
        if !(method.starts_with("[") && method.ends_with("]")) {
            let abi_str = format!("[{}]", method);
            json_parse = serde_json::from_str(&abi_str);
        } else {
            json_parse = serde_json::from_str(&method);
        }
        let abi: Abi = json_parse.map_err(|e| {
            WrapperError::SerdeError(format!("Failed to parse ABI. {:?}", e))
        })?;
        let (_, functions): (&String, &Vec<Function>) = abi.functions.iter().next().unwrap();
        let function: Function = functions.get(0).unwrap().clone();
        Ok(function)
    }
}