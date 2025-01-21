use hex::{decode, encode};

fn repeating_key_xor(plaintext: &str, key: &str) -> Vec<u8> {
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();

    plaintext
        .bytes()  // Use `bytes()` to get the byte representation of the plaintext
        .enumerate()
        .map(|(i, byte)| {
            let key_byte = key_bytes[i % key_len];
            byte ^ key_byte  // XOR the byte with the corresponding key byte
        })
        .collect::<Vec<u8>>()  // Collect the result as a vector of bytes
}

#[cfg(test)]
mod test {
    use hex::ToHex;

    use crate::set_1_basics::challenge5::repeating_key_xor;
    
    #[test]
    fn test_c5() {
        let text = "Burning 'em, if you ain't quick and nimble\n\
                    I go crazy when I hear a cymbal";
        let key = "ICE";

        let encrypted_bytes = repeating_key_xor(text, key);
        let encrypted_hex = hex::encode(encrypted_bytes);  // Convert the resulting bytes to hex

        println!("{}", encrypted_hex);

        assert_eq!(
            encrypted_hex,
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }
}
