use rand::Rng;
use crate::input::get_user_input;

pub const CHAR_SET: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub fn run() {
    let game_set = init();
    play(20);
    print_chars(&game_set);
}

fn init() -> [char; 5] {
    println!("---MasterMind---");
    let mut new_game_set: [char; 5] = Default::default();

    for i in 0..5 {
        let random = rand::thread_rng().gen_range(0..8);
        new_game_set[i] = CHAR_SET[random];
    }

    new_game_set
}

fn play(max_tries: u8) {
    let mut tries: u8 = 1;

    while tries <= max_tries {
        println!("Versuch {}", tries);
        let guess = get_user_input();
        print_chars(&guess);

        tries += 1;
    }
}

fn print_chars(chars: &[char; 5]) {
    for char in chars {
        print!("{}", char)
    }
}