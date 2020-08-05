use std::process::Command;
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
    g.test();

    let mut input = String::new();
    while input.as_str().trim() != "<ESC>" {
        input = read_input();
        match input.as_str().trim() {
            "n/N" => g.test(),
            "<RET>" => g.test(),
            "u/U" => g.test(),
            "<ESC>" => (),
            a => match a.parse::<u64>() {
                Ok(ok) => select_column(ok),
                Err(_) => println!("No es un valor válido\nValores válidos: <ESC>, <RET>, u/U, n/N, 0, 1, 2, 3,4, 5, 6")
            }
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

fn select_column(c: u64) {
    if c < 7 {
        println!("Columna: {}", c);
    } else {
        println!("No es un valor válido\nValores válidos: <ESC>, <RET>, u/U, n/N, 0, 1, 2, 3,4, 5, 6");
    }  
}
