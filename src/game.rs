mod rest_game;
// cuando se usa la mod name; le indica a rust que importe y carge el cuero de este archivo src/game/rest_game

#[derive(Debug)]
pub struct GameModel {
    pub name: String,
    points: u32,
}

pub fn new_game(name: &str) -> GameModel {
    return GameModel {
        name: name.to_string(),
        points: 0,
    };
}

pub fn init() {
    let obj_game = crate::game::new_game("La bolita");
    crate::game::rest_game::hosting::test_rest();

    println!("Iniciando desde el juego: {}", obj_game.name);
}
