mod test {

    use crate::utils::pcks::pad_pcks7;

    #[test]
    fn test_c9() {
      let msg = "YELLOW SUBMARINE";

      let pad_str = pad_pcks7(msg, 20);

      assert_eq!(pad_str, "YELLOW SUBMARINE\x04\x04\x04\x04");
      
    }
}
