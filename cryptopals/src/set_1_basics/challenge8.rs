mod test {

    use aes::cipher::{BlockDecrypt, KeyInit};
    use aes::Aes128;
    use base64;
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::Read;

    use crate::utils::file::read_file;

    #[test]
    fn test_c8() {
        let key = b"YELLOW SUBMARINE";
        let file = read_file("data/8.txt").expect("Error reading file");
        let data_decoded = base64::decode(file).unwrap();

        let mut seen_blocks = HashSet::new();
        let mut ecb_detected = false;

        for chunk in data_decoded.chunks(16) {
            if !seen_blocks.insert(chunk) {
                ecb_detected = true;
                println!("ECB detected! Repeated block: {:?}", chunk);
            }
        }

        assert!(ecb_detected, "No ECB mode detected.");
    }
}
