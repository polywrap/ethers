use ethabi::token::Tokenizer;
use ethers_core::{
    abi::{Param, Token, encode, AbiParser, HumanReadableParser},
    types::{Bytes}
};

pub fn encode_params(types: Vec<String>, values: Vec<String>) -> Vec<u8> {
    let tokens: Vec<Token> = values
        .iter()
        .zip(types.iter())
        .map(|(arg, t)| {
            let kind = HumanReadableParser::parse_type(&t).unwrap();
            ethabi::token::LenientTokenizer::tokenize(&kind, arg).unwrap()
        })
        .collect();
    let bytes = encode(&tokens);
    bytes
}

pub fn encode_function(method: &str, args: &Vec<String>) -> Bytes {
    let function = AbiParser::default().parse_function(method).unwrap();
    let tokens: Vec<Token> = tokenize_values(args, &function.inputs);
    let bytes: Bytes = function.encode_input(&tokens).unwrap().into();
    bytes
}

pub fn decode_function(method: &str, data: Vec<u8>) -> Vec<Token> {
    let function = AbiParser::default().parse_function(method).unwrap();
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
        .map(|(param, arg)| ethabi::token::LenientTokenizer::tokenize(&param.kind, arg).unwrap())
        .collect()
}