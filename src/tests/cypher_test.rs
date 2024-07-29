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
    //
    // #[test]
    // fn test_cesar_decrypt(){
    //     use ymcrust::cesar_decrypt;
    //     let text = "Mjqqt, Btwqi!";
    //     let key = 5;
    //     let result = cesar_decrypt(text, key);
    //     assert_eq!(result, "Hello, World!");
    // }



}