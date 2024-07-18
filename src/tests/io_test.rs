// File: io_test.rs

#[cfg(test)]
mod tests {

    //use super::*;


    //test  pas encore optimal
    #[test]
    fn test_input(){
        use ymcrust::input;
        let input_test = input();
        let result = input_test;
        assert_eq!(result,"YMCRUST");
    }

    //test  pas encore optimal
    #[test]
    fn test_input_numb(){
        use ymcrust::input_numb;
        let input_test = input_numb();
        let result = input_test;
        assert_eq!(result,5);
    }

    #[test]
    fn test_random(){
        use ymcrust::random;
        let random = random(1,10);
        let result = random;
        assert_eq!(result, if result >= 1 && result <= 10 {result}else{0});
    }
}
