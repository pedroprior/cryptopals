#[cfg(test)]
mod test {

    fn hamming_distance(str_a: &str, str_b: &str) -> usize {

        if str_a.len() != str_b.len() {
            return 0;
        }
    
        let mut distance = 0;
    
        for (c1, c2) in str_a.bytes().zip(str_b.bytes()) {
            if c1 != c2 {
                distance += (c1 ^c2).count_ones() as usize;
            }
        }
    
        distance
    }
    

    #[test]
    fn test_c6() {
        
        let str_a: &str = "this is a test";
        let str_b: &str = "wokka wokka!!!";

        println!("{}", hamming_distance(str_a, str_b));

    }
}