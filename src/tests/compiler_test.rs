
#[cfg(test)]


mod tests {

    #[test]
    fn test_lexer() {
        use ymcrust::lexer;
        let result = lexer("2 + 2");
        assert_eq!(result, vec!["2", "+", "2"]);
    }




}