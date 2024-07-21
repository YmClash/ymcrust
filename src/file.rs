use std::fs::File;
use std::io::{BufRead,BufReader,Read};
use std ::io::Error;



// File Manupulation Module


// #001

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


pub fn read_file_2(path:&str) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = line.unwrap();
        println!("{}",line);
    }

}