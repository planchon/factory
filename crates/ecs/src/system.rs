use crate::entity_manager::EntityManager;

pub trait System {
    fn update(&mut self, delta_time: f32, entity_manager: &mut EntityManager);
}
