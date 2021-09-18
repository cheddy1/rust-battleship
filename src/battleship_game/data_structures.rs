// This enum needs to derive several traits
// The first two are copy and clone, they always come as a pair
// We need those because to create an array of any var T with n elements ([X; n]) the type of X
// needs to by copyable
// We need PartialEq to be able to compre different enum objects without using a match statement
// just cleans up the code a little bit
#[derive(Copy, Clone, PartialEq)]
pub enum WaterSquare
{
    Empty,
    Hit,
}

pub enum Players
{
    PlayerOne,
    PlayerTwo,
}

pub enum FireState
{
    Hit,
    Miss,
}
