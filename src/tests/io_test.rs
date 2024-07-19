// File: io_test.rs


#[cfg(test)]
mod tests {




    //test  pas encore optimal
    #[test]

    fn test_input(){
        use ymcrust::input;
        // println!("Enter a string : YMCRUST");
        let input_test = input();
        let result = input_test;
        assert_eq!(result,"YMCRUST");
    }

    //test  pas encore optimal
    #[test]

    fn test_input_numb(){
        use ymcrust::input_numb;
        // println!("Enter a number : 5");
        let input_test = input_numb();
        let result = input_test;
        assert_eq!(result,5);
    }

    #[test]

    fn test_pause(){
        use ymcrust::pause;
        let result = pause(2);
        assert_eq!(result,());
    }
}
