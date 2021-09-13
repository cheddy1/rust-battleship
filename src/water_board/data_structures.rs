// We need this enum to implement the copy trait because when we initialize an array
// All with Empty squares, it needs to by copyable
#[derive(Copy, Clone)]
pub enum WaterSquare
{
    Empty,
    Hit,
}
