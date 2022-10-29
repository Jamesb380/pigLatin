
#![allow(non_snake_case)]
use std::io;
use is_vowel::IsRomanceVowel;


fn main() {
    println!("Translate a phrase to Pig Latin");

    println!("Enter your phrase");
    let mut phrase = String::new();
    io::stdin().read_line(&mut phrase).expect("Unable to read Input");
    let p = phrase.to_string();
    println!("{:?}", pig_Latin(p));
}


fn pig_Latin(s:String)-> String{
    const AY: &'static str = "ay";
    const HAY: &'static str = "hay";
    let mut phrase = String::new();

    for word in s.split_whitespace(){

        let c: Vec<char> = word.chars().collect();
        if !c[0].is_romance_vowel(){

            let word_ay =format!("{}{}-{} ", &word[1..], &word[0..1], AY);
            phrase.push_str(&word_ay);
                             }
        else {
            let word_ay = format!("{}-{} ",&word,HAY);
            phrase.push_str(&word_ay);
        }
                            
        }

    phrase
}