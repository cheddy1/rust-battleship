#[derive(Copy, Clone)]
pub enum WaterSquare
{
    Empty,
    Hit,
}

pub enum ShipType
{
    BattleShip,
    Carrier,
    Cruiser,
    Submarine,
    Destroyer,
}

pub enum Players
{
    PlayerOne,
    PlayerTwo,
}
