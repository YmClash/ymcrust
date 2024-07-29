


// Cypher module

//001


pub fn cesar_encrypt(text: &str, key:u8)  -> String {
    let mut msg_cypher  = String::new();
    for character in text.chars(){
        if character.is_alphabetic(){
            let base = if character.is_ascii_lowercase() { b'a' } else { b'A' };
                //le vrai algorithme de cesar;
            let cypher  = ((character as u8 - base + key) %  26) + base;
            msg_cypher.push(cypher as char);
        }else {
            msg_cypher.push(character);
        }
    }
    return msg_cypher;


}



pub fn cesar_decrypt(text:&str,key:u8)  -> String {
    let mut msg_decrypt = String::new();
    for character in text.chars(){
        if character.is_alphabetic(){
            let base = if character.is_ascii_lowercase(){ b'a' } else { b'A' };
            let decypher = ((character as u8 - base + 26 - key) % 26) + base;
            msg_decrypt.push(decypher as char);
        }
        else {
            msg_decrypt.push(character);
        }
    }
    return msg_decrypt;
}
