use ethers_core::utils::keccak256;

pub fn get_checksum_address(address: &str) -> String {
    let address = address.trim_start_matches("0x").to_lowercase();
    let mut chars: Vec<char> = address.chars().collect();

    let expanded: Vec<u8> = chars.iter().map(|c| *c as u8).collect();

    let hashed = keccak256(&expanded);

    for i in (0..40).step_by(2) {
        if hashed[i >> 1] >> 4 >= 8 {
            chars[i] = chars[i].to_ascii_uppercase();
        }
        if (hashed[i >> 1] & 0x0f) >= 8 {
            chars[i + 1] = chars[i + 1].to_ascii_uppercase();
        }
    }

    "0x".to_string() + &chars.into_iter().collect::<String>()
}
