use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs:&mut SubWorld, commands:&mut CommandBuffer){
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query()
        .filter(component::<Player>());

    players.iter(ecs).for_each(|pos| player_pos = *pos);  //iter(ecs) 可能会修改查询器的内部状态（例如，缓存结果或标记访问），因此需要mut players。
    let mut enemies = <(Entity, &Point)>::query()  //Entity 是一个标识符，用于唯一标识 ECS 系统中的每个实体。它不是普通的用户定义组件，而是框架内置的元数据。特性：每个实体都会自动分配一个 Entity 标识符。Entity 标识符不是用户显式添加的，而是框架在调用 ecs.push() 时自动生成和管理的,这里由于要删除掉实体，因此要通过实体中的该默认组件来删除实体。
        .filter(component::<Enemy>());

    enemies
        .iter(ecs)
        .filter(|(_,pos)| **pos==player_pos)
        .for_each(|(entity,_)|{
            commands.remove(*entity);
        });
}



//<&Point, &Player>::query() 会返回所有同时具有 Point 和 Player 的实体，并且返回的元组中会包含 &Point 和 &Player 两个引用。这种方法适用于你需要同时访问 Point 和 Player 的值。

//<&Point>::query().filter(component::<Player>()) 只返回具有 Point 的引用，但会根据 Player 组件进行筛选。如果你不需要访问 Player 的值，而仅仅关心 Player 的存在，那么使用 filter 是更高效、更简单的选择。

//第一种相当于我们可以找到具有Point、Player的实体的Point和Player的值。
//第二种相当于我们可以找到具有Point、Player组件的实体Point的值，这种要更高效一些。
//query()作用并不是找到某个实体，而是找到实体的某个组件以便于之后修改该组件内容。