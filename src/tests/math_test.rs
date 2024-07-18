// File: math_test.rs
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        use ymcrust::add;

        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]

    fn test_random(){
        use ymcrust::random;

        let random = random(1,10);
        let result = random;
        assert_eq!(result, if result >= 1 && result <= 10 {result}else{0});
    }
}

