//file: string_test.rs

//mod string_test;

#[cfg(test)]
mod tests {

    #[test]
    fn test_is_palindrome() {
        use ymcrust::is_palindrome;
        assert_eq!(is_palindrome("radar"), true);
        assert_eq!(is_palindrome("level"), true);
        assert_eq!(is_palindrome("world"), false);
        assert_eq!(is_palindrome("ressasser"), true);
        assert_eq!(is_palindrome("Et la marine va venir à Malte"), false);
        //assert_eq!(is_palindrome("a man a plan a canal panama"), true); // Test avec des espaces et majuscules
        assert_eq!(is_palindrome(""), true); // Test avec une chaîne vide
        assert_eq!(is_palindrome("12321"), true); // Test avec des chiffres
        assert_eq!(is_palindrome("12345"), false); // Test avec des chiffres
        assert_eq!(is_palindrome("あいいあ"), true); // Test avec des caractères Unicode
        assert_eq!(is_palindrome("あいえあ"), false); // Test avec des caractères Unicode
    }


    #[test]
    fn test_count_char() {
        use ymcrust::count_char;
        assert_eq!(count_char("Hello"), 5);
        assert_eq!(count_char("World"), 5);
        assert_eq!(count_char("YMCRUST"), 7);
        assert_eq!(count_char("123456789"), 9);
        assert_eq!(count_char("こんにちは"), 5); // test unicode caractere
    }

    #[test]
    fn test_reverse() {
        use ymcrust::reverse;
        assert_eq!(reverse("Hello"), "olleH");
        assert_eq!(reverse("World"), "dlroW");
        assert_eq!(reverse(""), "");
        assert_eq!(reverse("123456789"), "987654321");
        assert_eq!(reverse("こんにちは"), "はちにんこ"); // test unicode caractere
    }


}
