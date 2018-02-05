//increment the give nonce byte values by one
pub fn incr_nonce<T: Into<String>> (nonce: T) -> String {
    let c: Vec<u8> = nonce.into().as_bytes().into_iter().map(|x| x + 1).collect();
    String::from_utf8(c).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incr_test() {
        assert_eq!("23456789", incr_nonce("12345678"));
        assert_eq!("", incr_nonce(""));
        assert_eq!("ĥķĽ", incr_nonce("äöü"));
    }

}
