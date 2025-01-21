// Single-byte XOR cipher

use regex::Regex;

// Converts hex string to a UTF-8 string
fn hex2str(hex_str: &str) -> String {
    let bytes = hex::decode(hex_str).expect("Invalid hex string");
    String::from_utf8_lossy(&bytes).to_string()
}

// XORs a string with a single character
fn string_xor(s: &str, c: u8) -> String {
    s.chars()
        .map(|h| ((h as u8) ^ c) as char)
        .collect::<String>()
}

#[cfg(test)]
mod test {

    use super::hex2str;
    use super::string_xor;
    use super::Regex;

    #[test]
    fn test_c3() {
        let hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        // Regex to remove control characters
        let re = Regex::new(r"[\x00-\x1F]+").unwrap();

        for c in 0u8..=255 {
            let decoded_str = hex2str(hex);
            let xor_result = string_xor(&decoded_str, c);

            let pretty_result = re.replace_all(&xor_result, "");

            // If the result has a large portion of printable characters, it could be the correct decryption
            if pretty_result
                .chars()
                .filter(|&ch| ch.is_ascii_alphanumeric() || ch == ' ')
                .count()
                > 10
            {
                println!("XOR key 0x{:x}: {}", c, pretty_result);
            }
        }
    }
}
