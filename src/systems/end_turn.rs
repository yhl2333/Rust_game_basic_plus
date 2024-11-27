use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState){
    let new_state = match turn_state{
        TurnState::AwaitingInput =>return,  //因为这一步要等待玩家输入，所以是return，如果是TurnState::AwaitingInput => TurnState::PlayerTurn，那么每次tick状态机的状态都会变化
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput
    };
    
    *turn_state = new_state;
}