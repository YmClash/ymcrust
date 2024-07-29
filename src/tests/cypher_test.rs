#[cfg(test)]


mod tests {
    #[test]
    fn test_cesar_encrypt() {
        use ymcrust::cesar_encrypt;
        let text = "Hello, World!";
        let key = 5;
        let result = cesar_encrypt(text, key);
        assert_eq!(result, "Mjqqt, Btwqi!");
    }

    #[test]
    fn test_cesar_decrypt() {
        use ymcrust::cesar_decrypt;
        let text = "Mjqqt, Btwqi!";
        let key = 5;
        let result = cesar_decrypt(text, key);
        assert_eq!(result, "Hello, World!");
    }


    #[test]
    fn test_chacha_encrypt() {
        use ymcrust::chacha_encrypt;
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let text = "Hello, World!";
        let result = chacha_encrypt(&key, &nonce, text);
        assert_eq!(result,vec![62, 221, 140, 193, 207, 221, 29, 199, 47, 47, 6, 129, 114])
    }

    #[test]
    fn test_chacha_decrypt() {
        use ymcrust::{chacha_encrypt,chacha_decrypt};
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let text = vec![62, 221, 140, 193, 207, 221, 29, 199, 47, 47, 6, 129, 114];
        let plaintext = "Hello, World!";

        let cyphertext  = chacha_encrypt(&key, &nonce, plaintext);
        let result = chacha_decrypt(&key, &nonce, &text);
        assert_eq!(cyphertext,text);
        assert_eq!(result, plaintext.as_bytes().to_vec());
    }
}
