use std::fs::File;
use std::io::{BufRead,BufReader,Read};
use std ::io::Error;
//use polars::prelude::*;
//use polars::prelude::DataType::String;
// File Manupulation Module


// #001

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


// #002
pub fn read_show_file(path:&str) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = line.unwrap();
        println!("{}",line);
    }

}

// #003

pub fn read_file_length(file_path:&str) -> usize {
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    return content.len();
}

//csv file
//
// pub fn read_csv_with_polars(path: &str) -> Result<DataFrame> {
//     let df = CsvReader::from_path(path)?
//         .has_header(true)
//         .finish()?;
//     Ok(df)
// }
//




// #004
//
// pub fn read_and_show_csv_file(path: &str) -> () {
//     let file = File::open(path).unwrap();
//     let mut reader = ReaderBuilder::new().from_reader(file);
//     for record in reader.records(){
//         let record = record.unwrap();
//         println!("{:?}",record);
//     }
// }
//
//
