mod water_board;

fn main()
{
    let my_board = crate::water_board::WaterBoard::new_board();     
    my_board.print_board();
}
