use ecs::world::World;
use ecs::{entity_manager::EntityManager, system::System};
use ecs_macros::Component;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Weight {
    value: f32,
}

struct IncreasePositionSystem;

impl System for IncreasePositionSystem {
    fn update(&mut self, delta_time: f32, entity_manager: &mut EntityManager) {
        let positions = entity_manager.borrow_components_mut::<Position>();
        for position in positions.iter_mut() {
            position.x += 1.0;
            position.y += 1.0;
        }
    }
}

struct SpeedSystem;
impl System for SpeedSystem {
    fn update(&mut self, delta_time: f32, entity_manager: &mut EntityManager) {
        let entities = entity_manager.query_entities_pair::<Velocity, Position>();

        if entities.is_none() {
            return;
        }

        for entity in entities.unwrap().iter() {
            let (velocity, position) = entity_manager
                .borrow_components_pair_for_entity::<Velocity, Position>(*entity)
                .unwrap();
            position.x += velocity.x;
            position.y += velocity.y;
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn simple_one_entity_world() {
        let mut world = World::new();

        world.register_component::<Position>();

        let first_entity = world.create_entity();

        world.add_component_to_entity(first_entity, Position { x: 0.0, y: 0.0 });

        world.register_system(IncreasePositionSystem);

        let position = world
            .borrow_component_from_entity::<Position>(first_entity)
            .unwrap();
        assert_eq!(position.x, 0.0);
        assert_eq!(position.y, 0.0);

        world.update();

        let position = world
            .borrow_component_from_entity::<Position>(first_entity)
            .unwrap();
        assert_eq!(position.x, 1.0);
        assert_eq!(position.y, 1.0);
    }

    #[test]
    fn multiple_entities_world() {
        let mut world = World::new();

        world.register_component::<Position>();
        world.register_component::<Weight>();

        let first_entity = world.create_entity();
        let second_entity = world.create_entity();
        let third_entity = world.create_entity();

        world.add_component_to_entity(first_entity, Position { x: 0.0, y: 0.0 });
        world.add_component_to_entity(second_entity, Position { x: 10.0, y: 10.0 });
        world.add_component_to_entity(third_entity, Weight { value: 10.0 });

        world.register_system(IncreasePositionSystem);

        let position = world
            .borrow_component_from_entity::<Position>(first_entity)
            .unwrap();
        assert_eq!(position.x, 0.0);
        assert_eq!(position.y, 0.0);

        let position = world
            .borrow_component_from_entity::<Position>(second_entity)
            .unwrap();
        assert_eq!(position.x, 10.0);
        assert_eq!(position.y, 10.0);

        // world.update() should change the position of the entity with a Position component
        // increment the position by 1
        world.update();

        let position = world
            .borrow_component_from_entity::<Position>(first_entity)
            .unwrap();
        assert_eq!(
            position.x, 1.0,
            "first entity x position should be updated by the IncreasePositionSystem"
        );
        assert_eq!(
            position.y, 1.0,
            "first entity y position should be updated by the IncreasePositionSystem"
        );

        let position = world
            .borrow_component_from_entity::<Position>(second_entity)
            .unwrap();
        assert_eq!(
            position.x, 11.0,
            "second entity x position should be updated by the IncreasePositionSystem"
        );
        assert_eq!(
            position.y, 11.0,
            "second entity y position should be updated by the IncreasePositionSystem"
        );

        let position = world.borrow_component_from_entity::<Position>(third_entity);
        assert_eq!(
            position.is_none(),
            true,
            "third entity should not have a Position component"
        );
    }

    #[test]
    fn multiple_components() {
        let mut world = World::new();

        world.register_component::<Position>();
        world.register_component::<Velocity>();

        let first_entity = world.create_entity();
        let second_entity = world.create_entity();

        world.add_component_to_entity(first_entity, Position { x: 0.0, y: 0.0 });
        world.add_component_to_entity(second_entity, Position { x: 10.0, y: 10.0 });
        world.add_component_to_entity(second_entity, Velocity { x: 1.0, y: 1.0 });

        world.register_system(SpeedSystem);

        let position = world
            .borrow_component_from_entity::<Position>(first_entity)
            .unwrap();

        assert_eq!(position.x, 0.0);
        assert_eq!(position.y, 0.0);

        let position = world
            .borrow_component_from_entity::<Position>(second_entity)
            .unwrap();
        assert_eq!(position.x, 10.0);
        assert_eq!(position.y, 10.0);

        // world.update() should change the position of the entity with a Position and Velocity component
        // increment the position by the velocity
        world.update();

        // first entity do not have a Velocity component, so it should not be updated
        let position = world
            .borrow_component_from_entity::<Position>(first_entity)
            .unwrap();
        assert_eq!(
            position.x, 0.0,
            "first entity x position should not be updated"
        );
        assert_eq!(
            position.y, 0.0,
            "first entity y position should not be updated"
        );

        let position = world
            .borrow_component_from_entity::<Position>(second_entity)
            .unwrap();
        assert_eq!(
            position.x, 11.0,
            "second entity x position should be updated by the velocity"
        );
        assert_eq!(
            position.y, 11.0,
            "second entity y position should be updated by the velocity"
        );
    }
}
