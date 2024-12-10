// Convert hex to base64.

use base64::{engine::general_purpose, Engine};

pub fn hex_to_base64(hex: String) -> String {
    let mut bytes = Vec::new();
    for i in 0..(hex.len() / 2) {
        let res = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => print!("Problems with hex: {}", e),
        };
    }

    return general_purpose::STANDARD.encode(&bytes);
}

#[cfg(test)]
mod test {

    use super::hex_to_base64;

    #[test]
    fn test_c1() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

        println!("{}", hex_to_base64(hex.to_string()));
    }
}
