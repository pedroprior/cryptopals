mod test {
    use aes::{cipher::KeyInit, Aes128};

    use crate::utils::{aes128::{aes_128_cbc_decrypt, aes_128_cbc_encrypt}, file::read_file};


    #[test]
    fn test_c10() {

        let key = "YELLOW SUBMARINE";

        let msg = "This is some secret message. Do not reveal. I mean really this is some secret..duh! Why would you want to reveal it anyway.";

        let iv = "\x00".repeat(16);

        let encrypted_hex_message = aes_128_cbc_encrypt(msg, key, &iv);

        let decrypted_msg = aes_128_cbc_decrypt(encrypted_hex_message.as_str(), key, iv.as_str());

        assert_eq!(msg, decrypted_msg);

    }
}
