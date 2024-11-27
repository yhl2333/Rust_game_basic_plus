use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health{current: 20, max: 20}
    ));
}


pub fn spawn_monster(ecs: &mut World, pos: Point, rng: &mut RandomNumberGenerator){

    let p = match rng.range(0, 10){
        1..=7 =>goblin(),
        _ =>orc()
    };
    

    ecs.push((
        Enemy,
        MovingRandomly,
        pos,
        Render{
            color: ColorPair::new(WHITE,BLACK),
            glyph: p.2
        },
        Health{current: p.0, max: p.0},  //三元组推导为p, p.0, p.1, p.2分别为三元组的三个元素。
        Name(p.1)
    )

    );
}


fn goblin() -> (i32, String, FontCharType){
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType){
    (4, "Orc".to_string(), to_cp437('o'))
}


