use std::env;
use std::io;
mod game;
mod table;
mod deck;
mod card;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut seed = 0;

    if args.len() > 1 {
        let test = args[1].parse::<u64>();
        match test {
            Ok(ok) => seed = ok,
            Err(_) => println!("El argumento no es un numero por lo tanto se utilizará un seed aleatorio")
        }
    }
    
    let mut g = game::Game::new(seed);

    let mut input = String::new();

    while input.as_str().trim() != "<ESC>" {

        input = read_input();

        match input.as_str().trim() {
            "n/N" => g.test(),
            "<RET>" => g.test(),
            "u/U" => g.test(),
            "<ESC>" => (),
            _ => println!("No es un comando válido"),
        }
    }
    
    // g.test();

    /*
    for i in 0..26 {
        match g.draw_card() {
            Ok(n)  => println!("Hay cartas"),
            Err(n) => println!("No hay cartas"),
        };
    }*/
    //game::start();
}

fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            input
        }
        Err(_) => {
            String::from("Error")
        }
    }
}
