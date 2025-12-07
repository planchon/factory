use crate::{component::Component, entity_manager::EntityManager, system::System};

pub struct World {
    entity_manager: EntityManager,
    systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            systems: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> usize {
        self.entity_manager.create_entity()
    }

    pub fn register_component<T: 'static + Component>(&mut self) -> &mut Self {
        self.entity_manager.register_component::<T>();
        self
    }

    pub fn register_system<T: 'static + System>(&mut self, system: T) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: T,
    ) -> &mut Self {
        self.entity_manager
            .add_component_to_entity(entity_id, component);
        self
    }

    pub fn borrow_component_from_entity<T: 'static + Component>(
        &self,
        entity_id: usize,
    ) -> Option<&T> {
        self.entity_manager
            .borrow_component_for_entity::<T>(entity_id)
    }

    pub fn update(&mut self) {
        let delta_time = 1.0 / 60.0;
        for system in self.systems.iter_mut() {
            system.update(delta_time, &mut self.entity_manager);
        }
    }
}
