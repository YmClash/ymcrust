
#[cfg(test)]


mod tests {

    #[test]
    pub fn test_lexer() {
        use ymcrust::lexxer;
        use ymcrust::Token::{Number, Plus};
        let source_code = "5 + 7   12  ";
        let result = lexxer(source_code);
        assert_eq!(result, vec![Number(5), Plus, Number(7), Number(12)]);
    }






}