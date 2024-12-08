// Write a function that takes two equal-length buffers and produces their XOR combination. 

use hex::{decode, encode};

pub fn fixed_xor(bytes_1: Vec<u8>, bytes_2: Vec<u8>) -> String {

    let xor_bytes: Vec<u8> = bytes_1
    .iter()
    .zip(bytes_2.iter())
    .map(|(&b1, &b2)| b1 ^ b2)
    .collect();

    return encode(xor_bytes);
}


fn main() {
    let buffer_a = decode("1c0111001f010100061a024b53535009181c").unwrap();
    let buffer_b = decode("686974207468652062756c6c277320657965").unwrap();

    println!("{}", fixed_xor(buffer_a, buffer_b));

}