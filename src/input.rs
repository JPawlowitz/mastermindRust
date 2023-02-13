use std::io;
use std::ops::Index;
use crate::game::CHAR_SET;

pub fn get_user_input() -> [char; 5] {
    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        if input.chars().count() == 6 {
            let mut valid = true;
            for char in input.chars().take(5) {
                if !CHAR_SET.contains(&char) {
                    valid = false;
                    break;
                }
            }

            if valid {
                let chars_in_input: Vec<char> = input.chars().collect();
                let mut char_result: [char; 5] = Default::default();

                for i in 0..5 {
                    char_result[i] = chars_in_input[i];
                }

                return char_result;
            }
        }

        println!("Input fehlerhaft!");
        input.clear();
    }
}