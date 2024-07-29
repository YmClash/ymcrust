//use std::io::Read;
use chacha20::cipher::{NewCipher, StreamCipher};
use chacha20::{ChaCha20};

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

//CHACHA20

//002


pub fn chacha_encrypt(key: &[u8;32], nonce: &[u8;12], text: &str) -> Vec<u8> {
    let mut cypher = ChaCha20::new(key.into(),nonce.into());
    let mut cypher_text = text.as_bytes().to_vec();
    cypher.apply_keystream(&mut cypher_text);
    return cypher_text;
}



pub fn chacha_decrypt(key: &[u8;32], nonce: &[u8;12], text: &[u8]) -> Vec<u8> {
    let mut cypher = ChaCha20::new(key.into(),nonce.into());
    let mut decrypt_text = text.to_vec();
    cypher.apply_keystream(&mut decrypt_text);
    return decrypt_text;

}


