#[derive(Copy,Clone,Debug,PartialEq)]
pub enum TurnState{
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Element{
    Fire,
    Water,
    Grass,
    Eletric,

}