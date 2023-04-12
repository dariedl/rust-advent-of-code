use specs::{
    Builder, Component, Dispatcher, DispatcherBuilder, Join, ReadStorage, System, VecStorage,
    World, WorldExt, WriteStorage,
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

fn dispatch_builder() -> Dispatcher<'static, 'static> {
    DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .with(HelloWorld, "hello_updated", &["update_pos"])
        .build()
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
    let mut dispatcher = dispatch_builder();
    dispatcher.dispatch(&mut world);
}

#[cfg(test)]
mod tests {
    use specs::{Builder, Entities, Join, ReadStorage, RunNow, System, WorldExt};

    use crate::{create_world, dispatch_builder, Position, Velocity};

    #[test]
    fn first_test() {
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

        let mut dispatcher = dispatch_builder();
        dispatcher.dispatch(&mut world);

        struct CheckSys;

        impl<'a> System<'a> for CheckSys {
            type SystemData = (Entities<'a>, ReadStorage<'a, Position>);

            fn run(&mut self, (entities, pos): Self::SystemData) {
                for (entity, pos) in (&entities, &pos).join() {
                    println!("Entity: {:?}", entity);
                    if entity.id() == 0 {
                        assert_eq!(pos.x, 4.0);
                        assert_eq!(pos.y, 7.0);
                    } else if entity.id() == 1 {
                        assert_eq!(pos.x, 2.005);
                        assert_eq!(pos.y, 5.01);
                    } else {
                        panic!();
                    }
                }
            }
        }

        let mut check = CheckSys;
        System::setup(&mut check, &mut world);
        check.run_now(&world);
    }
}
