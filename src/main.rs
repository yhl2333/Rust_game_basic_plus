mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;


//use map::my_func;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;

    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub use crate::turn_state::*;
}



use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems:Schedule
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);//这里把一个玩家实体加入到ecs中

        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, pos, &mut rng));

        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.map); //这里用了move转移了资源
        resources.insert(Camera::new(map_builder.player_start));
        
        
        let mb = Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems:build_monster_scheduler(),
        };

        //println!("v's length is {}", map_builder.map.tiles.len());//borrow of moved value: `map_builder.map`

        mb
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();  //注释掉后，背景渲染会残留，大部分位置会被后来的渲染刷新，但地图之外的黑色部分则会出现渲染残留
        ctx.set_active_console(1);
        ctx.cls();   //注释掉后，第一层的怪会残留，但player不会残留，因为一直在最中心位置渲染
        ctx.set_active_console(2);
        ctx.cls();


        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput =>self.input_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn =>self.monster_systems.execute(&mut self.ecs, &mut self.resources),
        }

        render_draw_buffer(ctx).expect("Render error");




    }
}

fn main() -> BError {
    println!("Hello, world!");
    // let context = BTermBuilder::simple80x50()
    //     .with_title("Dungeon Crawler")
    //     .with_fps_cap(30.0)
    //     .build()?;

    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH*2, SCREEN_HEIGHT*2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
