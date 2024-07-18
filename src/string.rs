


// fonction qui retoune true si la chaine de caractère est un palindrome
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


// fonction  qui  compte le nombre de caractere

pub fn count_char(word:&str) -> usize{
    let count = word.chars().count();
    return count;
}


