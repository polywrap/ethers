use polywrap_wasm_rs::{BigInt};
use hex;
use crate::address::get_checksum_address;
use crate::regex::{matches_regex_array, matches_regex_bytes, matches_regex_number};

// convert a hex data string to bytes
fn get_bytes(value: &str) -> Result<Vec<u8>, String> {
    hex::decode(value).map_err(|e| format!("Invalid hex string: {}", e))
}

// convert bytes to a hex data string
fn hexlify(value: &[u8]) -> String {
    hex::encode(value)
}

// convert an array of byte arrays to a concatenated hex data string
fn concat(values: &Vec<Vec<u8>>) -> String {
    "0x".to_string() + &values.iter()
        .map(|v| hexlify(v))
        .collect::<Vec<String>>()
        .join("")
}

// get the byte length of a hex data string
fn data_length(value: &str) -> Result<usize, String> {
    get_bytes(value).map(|v| v.len())
}

fn to_utf8_bytes(value: &str) -> Vec<u8> {
    value.as_bytes().to_vec()
}

// Convert `value` to a twos-complement representation of
// `width` bits.
//
// The result will always be positive.
fn to_twos(value: &BigInt, width: usize) -> Result<BigInt, String> {
    let zero = BigInt::from(0);
    let one = BigInt::from(1);

    let limit = &one << (width - 1);

    let mut new_value = value.clone();

    if new_value < zero {
        new_value = -new_value;
        if new_value > limit {
            return Err(format!("NUMERIC_FAULT: result value too low in operation to_twos, value: {:?}", value));
        }
        let mask = (&one << width) - &one;
        let masked = (!new_value) & mask;
        return Ok(masked + &one);
    } else {
        if new_value >= limit {
            return Err(format!("NUMERIC_FAULT: result value too high in operation to_twos, value: {:?}", value));
        }
    }

    Ok(new_value)
}

fn to_be_array(value: &BigInt) -> Result<Vec<u8>, String> {
    if value < &BigInt::from(0) {
        return Err(format!("NUMERIC_FAULT: negative value in operation to_be_array, value: {:?}", value));
    }
    Ok(value.to_bytes_be().1)
}

fn zero_pad(data: &[u8], length: usize, left: bool) -> Result<String, String> {
    if length < data.len() {
        return Err(format!("BUFFER_OVERRUN: padding exceeds data length, buffer: {:?}, length: {}, offset: {}", data, length, length + 1));
    }

    let mut result = vec![0; length];
    if left {
        result[length - data.len()..].copy_from_slice(data);
    } else {
        result[..data.len()].copy_from_slice(data);
    }

    return Ok(hexlify(&result));
}

/// Return the `DataHexString` of data padded on the **left**
/// to length bytes.
///
/// This pads data the same as **values** are in Solidity
/// (e.g. `uint128`).
fn zero_pad_value(data: &str, length: usize) -> Result<String, String> {
    let bytes = get_bytes(data)?;
    zero_pad(&bytes, length, true)
}

/// Return the `DataHexString` of data padded on the **right**
/// to length bytes.
///
/// This pads data the same as **bytes** are in Solidity
/// (e.g. `bytes16`).
fn zero_pad_bytes(data: &str, length: usize) -> Result<String, String> {
    let bytes = get_bytes(data)?;
    zero_pad(&bytes, length, false)
}

fn _pack(type_: &str, value: &str, is_array: Option<bool>) -> Result<Vec<u8>, String> {
    match type_ {
        "address" => {
            if is_array.unwrap_or(false) {
                let padded = zero_pad_value(value, 32)?;
                return get_bytes(&padded);
            }
            let checksum_address: String = get_checksum_address(value);
            return get_bytes(&checksum_address);
        },
        "string" => return Ok(to_utf8_bytes(value)),
        "bytes" => return get_bytes(value),
        "bool" => {
            let value = if value.parse::<bool>().unwrap_or(false) { "0x01" } else { "0x00" };
            if is_array.unwrap_or(false) {
                let padded = zero_pad_value(value, 32)?;
                return get_bytes(&padded);
            }
            return get_bytes(value);
        },
        _ => (),
    };

    if let Some(cap) = matches_regex_number(type_) {
        let signed: bool = cap.0 == "int";

        let has_size = cap.1.len() > 0;
        let size: i32 = if has_size { cap.1.parse::<i32>().unwrap() } else { 256 };

        if !(!has_size || size.to_string() == cap.1 && size % 8 == 0 && size != 0 && size <= 256) {
            return Err(format!("invalid number type {}", type_));
        }

        let value = if signed {
            to_twos(&value.parse::<BigInt>().unwrap(), size as usize)?
        } else {
            value.parse::<BigInt>().unwrap()
        };

        let be = to_be_array(&value)?;
        let padded = zero_pad_value(&hexlify(&be), (size / 8) as usize)?;
        return get_bytes(&padded);
    }

    if let Some(cap) = matches_regex_bytes(type_) {
        let size = if cap.len() > 0 { cap.parse::<usize>().unwrap() } else { 0 };

        if size == 0 || size > 32 {
            return Err(format!("invalid bytes type {}", type_));
        }

        let data_length = data_length(value)?;

        if !(data_length == size) {
            return Err(format!("invalid value {} for type {}", value, type_));
        }

        if is_array.unwrap_or(false) {
            let padded = zero_pad_bytes(value, 32)?;
            return get_bytes(&padded);
        }
        return get_bytes(value);
    }

    if let Some(cap) = matches_regex_array(type_) {
        let base_type = cap.0;
        let count: usize = if cap.1.len() > 0 { cap.1.parse::<usize>().unwrap() } else { value.len() };

        if count != value.len() {
            return Err(format!("invalid array length {} for type {}", value, type_));
        }

        let value_array: Vec<&str> = value.split(',').collect(); // Assuming the input is a comma-separated string
        let mut result: Vec<Vec<u8>> = vec![];

        for value_element in value_array {
            let packed = _pack(&base_type, value_element, Some(true))?;
            result.push(packed);
        }


        return get_bytes(&concat(&result))
    }

    Err(format!("invalid type {}", type_))
}

pub fn solidity_pack(types: Vec<String>, values: Vec<String>) -> Result<String, String> {
    if types.len() != values.len() {
        return Err(format!("wrong number of values; expected {}", types.len()));
    }

    let mut tight: Vec<Vec<u8>> = vec![];
    for (index, type_) in types.iter().enumerate() {
        let packed = _pack(type_, &values[index], None)?;
        tight.push(packed);
    }
    Ok(concat(&tight))
}
