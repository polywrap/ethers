// r"^bytes([0-9]+)$"
pub fn matches_regex_bytes(input: &str) -> Option<String> {
    if input.starts_with("bytes") {
        let tail = &input[5..];
        if tail.chars().all(|c| c.is_digit(10)) {
            return Some(tail.to_string());
        }
    }
    None
}

// r"^(u?int)([0-9]*)$"
pub fn matches_regex_number(input: &str) -> Option<(String, String)> {
    let prefix = if input.starts_with("int") {
        "int"
    } else if input.starts_with("uint") {
        "uint"
    } else {
        return None
    };

    let tail = if prefix == "int" { &input[3..] } else { &input[4..] };
    if tail.chars().all(|c| c.is_digit(10)) {
        return Some((prefix.to_string(), tail.to_string()));
    }
    None
}

// r"^(.*)\[(.*)]$"
pub fn matches_regex_array(input: &str) -> Option<(String, String)> {
    if let Some(pos) = input.find('[') {
        if input.ends_with(']') {
            let base_type = &input[..pos];
            let count = &input[(pos + 1)..(input.len() - 1)];
            if count.chars().all(|c| c.is_digit(10)) {
                return Some((base_type.to_string(), count.to_string()));
            }
        }
    }
    None
}
