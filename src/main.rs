use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;
use std::env;
use std::io;
mod game;
mod table;
mod deck;
mod card;
mod rules;

use crossterm::event::{ read, Event, KeyCode, KeyEvent };

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

    // create file =============================================

    let mut i: usize = 0;

    while Path::new(format!("game_log_{}.txt", i).as_str()).exists() {
        i = i+1;
    }

    let file_name = format!("game_log_{}.txt", i);

    let mut file =  match File::create(file_name.as_str()) {
        Err(why) => panic!("No se pudo crear el archivo: {}", why),
        Ok(file) => file,
    };

    // =========================================================
    
    let mut g = game::Game::new(seed);
    let mut history: String = String::new();

    history.push_str(format!("\nHistoria del juego\nSeed: {}\n", seed).as_str());       
    history.push_str(format!("{}\n", table::table_log(&g.table)).as_str());

    g.print_table();

    loop {
        if g.check_done() { break }

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                println!("<RET>");
                g.draw_card();
                g.print_table();
                history.push_str("<RET>");
                history.push_str(format!("\n{}\n", table::table_log(&g.table)).as_str())
            },

            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                ..
            }) => {
                println!("<ESC>");
                history.push_str("<ESC>");
                break
            },

            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                match c {
                    'n' => {
                        println!("n/N");
                        seed = rand::thread_rng().gen();
                        g = game::Game::new(seed);
                        g.print_table();
                        history.push_str(format!("\n\nJuego Nuevo\nSeed: {}", g.seed).as_str());
                        history.push_str(format!("\n{}\n", table::table_log(&g.table)).as_str());
                    },
                    'u' => {
                        println!("u/U");
                        g.undo();
                        g.print_table();
                        history.push_str("u/U");
                        history.push_str(format!("\n{}\n", table::table_log(&g.table)).as_str())
                    },
                    a => match a.to_string().parse::<usize>() {
                        Ok(ok) => {
                            println!("{}", ok);
                            let sel = select_column(&mut g, ok);
                            history.push_str(format!("{}", ok).as_str());
                            if sel == 420 { history.push_str(" Ninguna") } else if sel != 42 { history.push_str(format!(" {}", sel).as_str()) } 
                            history.push_str(format!("\n{}\n", table::table_log(&g.table)).as_str())
                        },
                        Err(_) => {
                            history.push_str("\nNo es un valor válido\nValores válidos: <ESC>, <RET>, u/U, n/N, 1, 2, 3,4, 5, 6, 7");
                            println!("No es un valor válido\nValores válidos: <ESC>, <RET>, u/U, n/N, 1, 2, 3,4, 5, 6, 7")
                        }
                    }
                }
            },
            _ => {}
        }
    }

    if g.check_done() {
        println!("\nFelicidades has ganado el juego!\n")
    }
    println!("\nPresiona cualquier tecla para generar el log");
    read_input();

    match file.write_all(history.as_bytes()) {
        Err(why) => panic!("No se pudo escribir en el archivo : {}", why),
        Ok(_) => println!("\nSe creo el archivo {}", file_name),
    }

    println!("{}", history);
   
    println!("\nPresiona cualquier tecla para salir");
    read_input();
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