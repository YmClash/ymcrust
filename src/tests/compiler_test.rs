
#[cfg(test)]


mod tests {

    #[test]
    pub fn test_lexer() {
        use ymcrust::lexxer;
        let result = lexxer("2 + 2");
        assert_eq!(result, vec!["2", "+", "2"]);
    }




}