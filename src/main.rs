mod game;
mod table;
mod deck;
mod card;

fn main() {
    let mut g = game::Game::new(0);

    g.test();

    /*
    for i in 0..26 {
        match g.draw_card() {
            Ok(n)  => println!("Hay cartas"),
            Err(n) => println!("No hay cartas"),
        };
    }*/
    //game::start();
}
