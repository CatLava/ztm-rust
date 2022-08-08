//let word = String::from("hello world");
use std::collections::HashMap;

fn encryption_key(mut shift: u8) -> HashMap<char, char> {
    // make a collections of chars
    let alpha: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut keys = HashMap::new();
    for letter in &alpha{
        let index: usize = ((26 + shift) % 26).into();
        println!("{} : {}",letter, alpha[index]);
        keys.insert(letter.to_owned(), alpha[index]);
        shift += 1
    }
    println!("{:?}", keys);
    return keys
}
fn encrypt_string(phrase: String, keys: HashMap<char, char>) -> String {
    let mut estring = String::new();
    for letter in phrase.chars() {
        println!("{}", keys[&letter])
    }
    return phrase

}
fn main(){
    let word = String::from("HELLOWORLD");

    let keys = encryption_key(5);
    let new = encrypt_string(word, keys);
}