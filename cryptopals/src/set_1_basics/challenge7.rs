#[cfg(test)]
mod test {
    
    use base64::decode;
    use openssl::symm::{decrypt, Cipher};

    use crate::utils::file::read_file;

    #[test]
    fn test_c7() {
        
        let key = b"YELLOW SUBMARINE";
        let file = read_file("data/7.txt").expect("Error");
        
        let encrypted_data = decode(file).expect("Failed to decode base64");

        let decrypted_data = decrypt(Cipher::aes_128_ecb(), key, None, &encrypted_data).expect("Decryption failed!");

        let decrypted_str = String::from_utf8(decrypted_data).expect("Failed to convert decrypted data from UTF-8");

        println!("Decrypted text: {}", decrypted_str);
    }
}
