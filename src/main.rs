mod battleship_game;

fn main()
{
    let mut my_game = crate::battleship_game::BattleShipGame::begin();
    my_game.place_ships();
    my_game.play_game();
}
