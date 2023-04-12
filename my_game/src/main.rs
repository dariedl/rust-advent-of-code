use specs::{
    Builder, Component, DispatcherBuilder, Join, ReadStorage, System, VecStorage, World, WorldExt,
    WriteStorage,
};

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * 0.05;
            pos.y += vel.y * 0.05;
        }
    }
}

fn create_world() -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    world
}

fn dispatch_world(world: &mut World) {
    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .with(HelloWorld, "hello_updated", &["update_pos"])
        .build();

    dispatcher.dispatch(world);
}

fn main() {
    let mut world = create_world();

    // Only the second entity will get a position update,
    // because the first one does not have a velocity.
    world
        .create_entity()
        .with(Position { x: 4.0, y: 7.0 })
        .build();
    world
        .create_entity()
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 0.1, y: 0.2 })
        .build();

    dispatch_world(&mut world);
    world.maintain();
}

#[cfg(test)]
mod tests {
    use specs::{Builder, Join, WorldExt};

    use crate::{create_world, dispatch_world, Position, Velocity};

    #[test]
    fn mixed_create_merge() {
        let mut world = create_world();

        world
            .create_entity()
            .with(Position { x: 4.0, y: 7.0 })
            .build();
        world
            .create_entity()
            .with(Position { x: 2.0, y: 5.0 })
            .with(Velocity { x: 0.1, y: 0.2 })
            .build();

        dispatch_world(&mut world);
        world.maintain();
        let entities = &world.entities();
        let r = entities.join();
        // let five: Vec<_> = entities.create_iter().take(5).collect();
        r.for_each(|e| {
            println!("Entity: {:?}", e);
        });
    }
}
