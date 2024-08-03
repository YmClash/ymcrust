




#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    fn create_test_file(content:&str) ->String{
        let file_path = "test_file.txt";
        let mut file = File::create(file_path).expect("Error creating file");
        file.write_all(content.as_bytes()).expect("Error writing file");
        file_path.to_string()

    }

    #[test]

    fn test_read_file() {
        use ymcrust::read_file;
        let file_path = create_test_file("Hello, World!\nThis is a test file.");
        let result = read_file(&file_path).unwrap();
        //assert!(result.is_ok());
        assert_eq!(result, "Hello, World!\nThis is a test file.");
        //clean up
        std::fs::remove_file(file_path).expect("Error deleting file");
    }

    #[test]
    fn test_read_show_csv(){
        use ymcrust::read_and_show_csv_file;
        let path ="test.csv";
        let mut file = File::create(path).expect("Error creating file");
        writeln!((file), "name,age,city").expect("Error writing to file");

        let result = read_and_show_csv_file(path);
        assert!(result , vec!["name","age","city"]);

        std::fs::remove_file(path).unwrap();



    }


    //
    // #[test]
    //
    // fn test_read_file_2() {
    //     use ymcrust::read_file_2;
    //     let file_path = create_test_file("Hello, World!\nThis is a test file.");
    //
    //     let output = std::io::stdout();
    //     let mut output_handle = output.lock();
    //     let _output_bytes = std::io::BufWriter::new(&mut output_handle);
    //     let result = std::panic::catch_unwind(||{
    //         read_file_2(&file_path);
    //     });
    //
    //
    //     //clean up
    //     std::fs::remove_file(file_path).expect("Error deleting file");
    //     assert!(result.is_ok());
    // }
    //


}