mod battleship_game;

fn main()
{
    let my_game = crate::battleship_game::BattleShipGame::init_game(5);
    my_game.print_p1_board();
    my_game.print_p1_ships();
}
