// File: io_test.rs


#[cfg(test)]
mod tests {

    //test  pas encore optimal
    #[test]

    fn test_input(){
        use ymcrust::input;
        // println!("Enter a string : YMCRUST");
        let input_test = input("Enter a string : YMCRUST ");
        let result = input_test;
        assert_eq!(result,"YMCRUST");
    }

    //test  pas encore optimal
    #[test]

    fn test_input_numb(){
        use ymcrust::input_numb;
        // println!("Enter a number : 5");
        let input_test = input_numb("Enter a number : 5 ");
        let result = input_test;
        assert_eq!(result,5);
    }

    #[test]

    fn test_pause(){
        use ymcrust::pause;
        let result = pause(2);
        assert_eq!(result,());
    }

    #[test]

    fn test_type_of(){
        use ymcrust::type_of;
        let number = 42;
        assert_eq!(type_of(&number),"&i32");

        let nom = "YMCRUST";
        assert_eq!(type_of(nom),"&str");

        let hello = "Hello, world!";
        let string = hello.to_string();
        assert_eq!(type_of(&string), "&alloc::string::String");


        let array = vec![1,2,3,4,5];
        assert_eq!(type_of(&array),"&alloc::vec::Vec<i32>");

        let decimal = 52.74;
        assert_eq!(type_of(&decimal),"&f64");

        let boolean = true;
        assert_eq!(type_of(&boolean),"&bool");
        //
        let option : Option<i32> = Some(42);
        assert_eq!(type_of(&option),"&core::option::Option<i32>");
    }
}
