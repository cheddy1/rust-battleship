// This enum needs to derive several traits
// The first two are copy and clone, they always come as a pair
// We need those because to create an array of any var T with n elements ([X; n]) the type of X
// needs to by copyable
// We need PartialEq to be able to compre different enum objects without using a match statement
// just cleans up the code a little bit
#[derive(Copy, Clone, PartialEq)]
//Class definition for a water square containing two potential states, Empty and Hit. State will be
//Empty if it has not been hit by a player and Hit if it has been hit by a player
pub enum WaterSquare
{
    Empty,
    Hit,
}
//Class definition for the players containing two potential states, PlayerOne and PlayerTwo
pub enum Players
{
    PlayerOne,
    PlayerTwo,
}
//Class definition for the fire state containing two potential states, Hit or Miss. State will be Hit
//if a ship has been hit and Miss if a water square has been hit
pub enum FireState
{
    Hit,
    Miss,
}
