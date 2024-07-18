use std::io;
use std::time::Duration;
use std::thread;


// Input Module

// input string


pub fn input() ->String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let input  = input.trim().to_string();
    return input ;

}

//input number
pub fn input_numb() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let input  : i32= input.trim().parse().expect("Error");
    return input;
}


pub fn pause(sec:u64) {
    let _ = thread::sleep(Duration::from_secs(sec));
}