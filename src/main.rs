mod battleship_game;

fn main()
{
    let mut my_game = crate::battleship_game::BattleShipGame::init_game(2);
    my_game.test();
    my_game.print_p1_board();
    my_game.print_p1_ships();
}
