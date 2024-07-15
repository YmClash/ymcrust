use std::io;
use rand::{Rng,thread_rng};
use std::time::Duration;
use std::thread;



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

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


//generate random number b
pub fn random(start:i32,end:i32) ->i32 {
    let mut gen = thread_rng();
    let random = gen.gen_range(start..end);
    return random;

}

pub fn pause(sec:u64) {
    let _ = thread::sleep(Duration::from_secs(sec));
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    //test  pas encore optimal

    #[test]
    fn test_input(){
        let input_test = input();
        let result = input_test;
        assert_eq!(result,"YMCRUST");
    }

    //test  pas encore optimal
    #[test]
    fn test_input_numb(){
        let input_test = input_numb();
        let result = input_test;
        assert_eq!(result,5);
    }

    #[test]
    fn test_random(){
        let random = random(1,10);
        let result = random;
        assert_eq!(result, if result >= 1 && result <= 10 {result}else{0});
    }
}
