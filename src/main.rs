use rand::Rng;
use std::env;
use std::io;
mod game;
mod table;
mod deck;
mod card;
mod rules;
mod history;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut seed: u64 = 0;

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
    println!(" Después de terminar la partida se imprime un log");
    println!("===================================================");

    // generate seed
    if args.len() == 1 {
        seed = rand::thread_rng().gen();
    }
    
    let mut g = game::Game::new(seed);
    let mut h = history::History::new(g.seed);
    g.print_table();

    let mut input = String::new();
    let mut sel: usize = 0;
    while input.as_str().trim() != "<ESC>" && !g.check_done() {
        input = read_input();
        match input.as_str().trim() {
            "n/N" => {
                seed = rand::thread_rng().gen();
                g = game::Game::new(seed);
                h.add_play(("n/N".to_string(), sel, seed));
                g.print_table()
            },
            "<RET>" => {
                g.draw_card();
                h.add_play(("<RET>".to_string(), sel, seed));
                g.print_table();
            }
            "u/U" => {
                g.undo();
                h.add_play(("u/U".to_string(), sel, seed));
                g.print_table();
            },
            "<ESC>" => h.add_play(("<ESC>".to_string(), sel, seed)),
            a => match a.parse::<usize>() {
                Ok(ok) => {
                    sel = select_column(&mut g, ok);
                    h.add_play((ok.to_string(), sel, seed));
                },
                Err(_) => println!("No es un valor válido\nValores válidos: <ESC>, <RET>, u/U, n/N, 1, 2, 3,4, 5, 6, 7")
            }
        }
    }

    if input != "<ESC>" && g.check_done() {
        println!("Felicidades has ganado el juego!")
    }

    h.generate_log();
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

fn select_column(game: &mut game::Game, c: usize) -> usize {
    let mut sel: usize = 0;
    if c > 0 && c < 8 {
        sel = game.play_card(c-1);
        game.print_table();
    } else {
        println!("No es un valor válido\nValores válidos: <ESC>, <RET>, u/U, n/N, 1, 2, 3,4, 5, 6, 7");
    } 
    sel 
}

// seed terminable 7