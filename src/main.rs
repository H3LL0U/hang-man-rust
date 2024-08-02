/// ! A random word generator...


use rand::{self, Rng};
use rand::rngs::ThreadRng; 

use std::fs::File;

use std::io::{self, stdin, Read};
use std::iter;


fn clear_screen() {
    
    print!("\x1B[2J\x1B[1;1H");
    
}
fn random_word_gen() -> Result<String, io::Error> {
    
    let content: &str = include_str!("../word_list.txt");

    



    let mut rng: ThreadRng = rand::thread_rng();
    
    
    let words : Vec<&str> = content.split(", ").collect::<Vec<&str>>();
    let ran_val: usize = rng.gen_range(0..words.len()-1);
    let generated_word :String = words[ran_val].to_string();
    
    println!("generated word: {}", &generated_word);
    
    Ok(generated_word)
}


fn main_game() -> Result<bool,io::Error>{

    let mut atempts_left: u8 = 5;
    let mut user_input : String = String::new();
    let current_word: String = random_word_gen()?.to_ascii_lowercase();
    let mut guessed_letters: Vec<char> = Vec::new();
    let len_current_word: usize = current_word.len();
    let mut chars_found:usize =  0;
    

    let mut correctly_guessed_letters : String = iter::repeat('*').take(current_word.len()).collect();
    while atempts_left > 0 {
        
        let mut replaced = false;
        //get the word with the correctly guessed letters
        println!("atempts left : {}" , atempts_left);
        println!("{}", correctly_guessed_letters);
        user_input = String::new();
        let _ = stdin().read_line(&mut user_input);
        user_input = user_input.replace("\r\n","").to_string();





        //check if single char is inpputted
        if user_input.chars().count()!=1{
            println!("You did not enter a character");
            clear_screen();
            continue;
        }
        
        if !guessed_letters.contains(&user_input.chars().next().unwrap()){
            guessed_letters.push(user_input.chars().next().unwrap());
        }
        else {
            println!("You already guessed that letter!");
            clear_screen();
            continue;
        }
        

        //check if a character is in a word
        for (i,chr) in current_word.chars().enumerate(){
            if chr == user_input.chars().nth(0).unwrap() {
                correctly_guessed_letters.replace_range(i..i+1, &chr.to_string());
                chars_found+=1;
                replaced = true;
                
            }

        }
        
        if chars_found == len_current_word{
            println!("You won!");
            break;
        }

        if replaced == false{
            atempts_left -= 1;
           
        }
        clear_screen();
        
    }   

    Ok(true)
}

fn main() {

    let main_game_closing_state = main_game();
    match main_game_closing_state {
        Ok(_n) => println!("game closed succesfully!"),
        Err(_e) => println!("Something went wrong")
    }
}
