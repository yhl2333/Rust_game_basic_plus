use crate::prelude::*;


#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(ecs: &SubWorld,
                #[resource] mouse_pos: &Point,
                #[resource] camera: &Camera
)
{
    let mut positions = <(Entity, &Point, &Name)>::query();  //positions 是查询实例，而不是查询结果本身。你需要将 positions 定义为可变的 mut，因为执行迭代时会修改查询实例的内部状态（例如迭代器的游标位置）。即使查询本身是只读的，迭代的操作需要查询对象是可变的。
    let offset = Point::new(camera.left_x,camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions.iter(ecs)
        .filter(|(_,pos,_)| **pos == map_pos)
        .for_each(|(entity, _, name)|{
            let screen_pos = *mouse_pos * 4;

            let display = if let Ok(health) = ecs.entry_ref(*entity)
                .unwrap()
                .get_component::<Health>()  //返回一个Result枚举体，交给Ok()处理该枚举体，不是error结果给health。
                {
                    format!("{} : {} hp", name.0, health.current)
                }else{
                    name.0.clone()
                };

                draw_batch.print(screen_pos, &display);

        });
    draw_batch.submit(10100).expect("Batch_err");
}