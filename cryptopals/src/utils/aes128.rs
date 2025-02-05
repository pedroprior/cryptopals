use aes::{cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit}, Aes128};

use super::{pcks::pad_pcks7, xor_bytes::xor_bytes};

pub fn aes_128_cbc_encrypt(message: &str, key_str: &str, iv_str: &str) -> String {
    let padded_message = pad_pcks7(message, 16);
    let msg_bytes = padded_message.as_bytes();
    let iv = iv_str.as_bytes().to_vec();

    let key = GenericArray::clone_from_slice(key_str.as_bytes());
    let cipher = Aes128::new(&key);

    let mut encrypted_blocks: Vec<Vec<u8>> = Vec::new();

    (0..message.len()).step_by(16).for_each(|x| {

         // Take last encrypted block or IV for first block iteration
        let last = encrypted_blocks.last().unwrap_or(&iv);

        // XOR last encrypted block with current msg block & encrypt result
        let xor_block = xor_bytes(last, &msg_bytes[x..x + 16]);

        let mut block = GenericArray::clone_from_slice(&xor_block);

        cipher.encrypt_block(&mut block);

        encrypted_blocks.push(block.into_iter().collect::<Vec<u8>>());

    });

    return hex::encode(encrypted_blocks.into_iter().flatten().collect::<Vec<u8>>());
}


pub fn aes_128_cbc_decrypt(cipher_hex: &str, key_str: &str, iv_str: &str) -> String {
    let encrypted_bytes = hex::decode(cipher_hex).unwrap();
    let key = GenericArray::clone_from_slice(key_str.as_bytes());
    let iv = iv_str.as_bytes();
    let cipher = Aes128::new(&key);

    let mut decrypted_blocks: Vec<Vec<u8>> = Vec::new();
    (0..encrypted_bytes.len()).step_by(16).for_each(|x| {
        // Take last of encrypted block or IV in case of first block iteration
        let last = if x == 0 {
            &iv
        } else {
            &encrypted_bytes[x - 16..x]
        };

        // Decrypt AES
        let mut block = GenericArray::clone_from_slice(&encrypted_bytes[x..x + 16]);
        cipher.decrypt_block(&mut block);
        let decrypted_block = block.into_iter().collect::<Vec<u8>>();

        // XOR decrypted block with last encrypted block to undo xor during encryption
        let xor_block = xor_bytes(last, &decrypted_block);
        decrypted_blocks.push(xor_block);
    });

    // Get number of padding bytes applied during encryption & remove padding
    let padding_byte = *decrypted_blocks.last().unwrap().last().unwrap() as usize;
    decrypted_blocks
        .into_iter()
        .flatten()
        .take(encrypted_bytes.len() - padding_byte)
        .map(|x| x as char)
        .collect::<String>()
}
