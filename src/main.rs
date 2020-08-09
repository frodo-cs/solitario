use std::env;
use std::io;
mod game;
mod table;
mod deck;
mod card;
mod rules;

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
    println!("===================================================");
    println!(" SOLITARIO");
    println!("---------------------------------------------------");
    println!(" <ESC> - Salir");
    println!(" n/N — Juego nuevo");
    println!(" <RET> — Carta nueva");
    println!(" 1, 2, 3, 4, 5, 6 ó 7 - Columnas");
    println!(" u/U — Undo");
    println!("===================================================");

    let mut g = game::Game::new(seed);
    g.print_table();

    let mut input = String::new();
    while input.as_str().trim() != "<ESC>" && !g.check_done() {
        input = read_input();
        match input.as_str().trim() {
            "n/N" => {
                g = game::Game::new(0);
                g.print_table()
            },
            "<RET>" => {
                g.draw_card();
                g.print_table();
            }
            "u/U" => {
                g.undo();
                g.print_table();
            },
            "<ESC>" => (),
            a => match a.parse::<usize>() {
                Ok(ok) => select_column(&mut g, ok),
                Err(_) => println!("No es un valor válido\nValores válidos: <ESC>, <RET>, u/U, n/N, 1, 2, 3,4, 5, 6, 7")
            }
        }
    }

    if input != "<ESC>" && g.check_done() {
        println!("Felicidades!")
    }
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

fn select_column(game: &mut game::Game, c: usize) {
    if c > 0 && c < 8 {
        game.play_card(c-1);
        game.print_table();
    } else {
        println!("No es un valor válido\nValores válidos: <ESC>, <RET>, u/U, n/N, 1, 2, 3,4, 5, 6, 7");
    }  
}
