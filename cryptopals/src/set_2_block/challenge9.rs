mod test {

    pub fn pad_pcks7(message: &str, block_size: usize) -> String  {
        let padding_size = block_size - message.len() % block_size;
        let padding_char = padding_size as u8 as char;
        let padding: String = (0..padding_size).map(|_| padding_char).collect();
        format!("{}{}", message, padding)    

    }
  
    #[test]
    fn test_c9() {
      let msg = "YELLOW SUBMARINE";

      let pad_str = pad_pcks7(msg, 20);

      assert_eq!(pad_str, "YELLOW SUBMARINE\x04\x04\x04\x04");

    }
}
