fn hex2str(hex_str: &str) -> String {
    let bytes = hex::decode(hex_str).expect("Invalid hex string");
    String::from_utf8_lossy(&bytes).to_string()
}

fn string_xor(s: &str, c: u8) -> String {
    s.chars()
        .map(|h| ((h as u8) ^ c) as char)
        .collect::<String>()
}
