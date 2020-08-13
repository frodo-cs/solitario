use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use crate::game::Game;
use crate::table;

pub struct History {
    seed: u64,
    history: Vec<(String, usize, u64)>,
    file: File,
    name: String
}

impl History {
    pub fn new(seed: u64) -> History {
        let mut i: usize = 0;

        while Path::new(format!("game_log_{}.txt", i).as_str()).exists() {
            i = i+1;
        }

        let n = format!("game_log_{}.txt", i);

        History {
            seed: seed,
            history: vec![],
            file: match File::create(n.as_str()) {
                Err(why) => panic!("No se pudo crear el archivo: {}", why),
                Ok(file) => file,
            },
            name: n
        }
    }

    pub fn generate_log(&mut self){
        let mut s: String = String::new();
        let mut g = Game::new(self.seed);
        s.push_str(format!("\nHistoria del juego\nSeed: {}\n", self.seed).as_str());       
        s.push_str(format!("{}\n", table::table_log(&g.table)).as_str());
        for play in self.history.iter() {
            s.push_str(play.0.as_str());
            match play.0.as_str() {
                "n/N" => {
                    g = Game::new(play.2);
                    s.push_str(format!("\n\nJuego Nuevo\nSeed: {}", g.seed).as_str());
                    s.push_str(format!("\n{}\n", table::table_log(&g.table)).as_str())
                },
                "<RET>" => {
                    g.draw_card();
                    s.push_str(format!("\n{}\n", table::table_log(&g.table)).as_str())
                }
                "u/U" => {
                    g.undo();
                    s.push_str(format!("\n{}\n", table::table_log(&g.table)).as_str())
                },
                "<ESC>" => (),
                a => match a.parse::<usize>() {
                    Ok(ok) => {
                        if ok > 0 && ok < 8 {
                            g.play_history(ok-1, play.1); 
                            if play.1 == 420 { s.push_str(" Ninguna") } else if play.1 != 42 { s.push_str(format!(" {}", play.1).as_str()) } 
                            s.push_str(format!("\n{}\n", table::table_log(&g.table)).as_str()); 
                        } else {
                            s.push_str("\nNo es un valor válido\nValores válidos: <ESC>, <RET>, u/U, n/N, 1, 2, 3,4, 5, 6, 7")
                        }        
                    },
                    Err(_) => s.push_str("\nNo es un valor válido\nValores válidos: <ESC>, <RET>, u/U, n/N, 1, 2, 3,4, 5, 6, 7")
                }
            }
        }
        println!("{}", s);
        match self.file.write_all(s.as_bytes()) {
            Err(why) => panic!("No se pudo escribir en el archivo : {}", why),
            Ok(_) => println!("\nSe creo el archivo {}", self.name),
        }
    }

    pub fn add_play(&mut self, play: (String, usize, u64)){
        self.history.push(play)
    }
}