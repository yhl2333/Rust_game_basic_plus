use crate::prelude::*;



#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input( ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] key:&Option<VirtualKeyCode>, #[resource] turn_state: &mut TurnState,){   //以下三种资源都insert到了resource中了
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        if let Some(key) = key {
            let delta = match key{
                VirtualKeyCode::Left => Point::new(-1,0),
                VirtualKeyCode::Right => Point::new(1,0),
                VirtualKeyCode::Up => Point::new(0,-1),
                VirtualKeyCode::Down => Point::new(0,1),
                _=>Point::new(0,0),
            };

            players.iter(ecs).for_each(|(entity,pos)|{
                let destination = *pos +delta;
                commands.push(((),WantsToMove{entity: *entity, destination}));     //向实体添加组件，并且player的entity被绑定到了WantsToMove组件上

            });
            *turn_state = TurnState::PlayerTurn;

        }






}