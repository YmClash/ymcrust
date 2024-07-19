


// STRING MODULE TEST


// #001: fonction qui retoune true si la chaine de caractère est un palindrome
pub fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let chars: Vec<char> = s.chars().collect();
    let length = chars.len();

    for i in 0..length / 2 {
        if chars[i] != chars[length - 1 - i] {
            return false;
        }
    }

    return true
}


// #002: fonction qui compte le nombre de caractère dans une chaine de caractère

pub fn count_char(word:&str) -> usize{
    let count = word.chars().count();
    return count;
}




// 003 : fonction qui retourne la chaine de caractère inversée
pub fn reverse(word:&str) ->String{
    let chars:Vec<char> = word.chars().collect();
    let mut reverse_chars = chars;
    reverse_chars.reverse();
    let reverse :String =  reverse_chars.into_iter().collect();
    return reverse;

}


//YmC