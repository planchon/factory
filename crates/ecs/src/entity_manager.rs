use crate::component::Component;
use crate::component_manager::{
    ComponentManager, ComponentManagerTrait, cast_manager, cast_manager_mut,
};
use crate::entity::Entity;
use crate::query_manager::QueryManager;
use std::any::TypeId;
use std::collections::HashMap;
use std::mem::transmute;

pub struct EntityManager {
    entities: Entities,
    components_managers: HashMap<TypeId, Box<dyn ComponentManagerTrait>>,
    query_manager: QueryManager,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: Entities::new(),
            components_managers: HashMap::new(),
            query_manager: QueryManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> usize {
        self.entities.create()
    }

    pub fn register_component<T: 'static + Component>(&mut self) -> &mut Self {
        if !self.has_component_manager::<T>() {
            let type_id = TypeId::of::<T>();
            self.query_manager.register_component::<T>();
            self.components_managers
                .insert(type_id, Box::new(ComponentManager::<T>::new()));
        }

        self
    }

    pub fn borrow_component_for_entity<T: 'static + Component>(
        &self,
        entity_id: usize,
    ) -> Option<&T> {
        self.borrow_component_manager::<T>()
            .borrow_component_for_entity(entity_id)
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: T,
    ) -> &mut Self {
        if !self.has_component_manager::<T>() {
            panic!(
                "Component manager not found for type: {}",
                std::any::type_name::<T>()
            );
        }

        let bitmask = self.query_manager.get_bitmask_for_entity(entity_id);
        self.query_manager.remove_entity(entity_id);
        let component_bitmask =
            if let Some(bitmask) = self.query_manager.get_bit_for_component::<T>() {
                *bitmask
            } else {
                panic!(
                    "Component not found for type: {}",
                    std::any::type_name::<T>()
                );
            };

        let new_bitmask = bitmask | component_bitmask;
        self.query_manager.add_entity(entity_id, new_bitmask);

        self.borrow_component_manager_mut::<T>()
            .add(entity_id, component);

        self
    }

    fn has_component_manager<T: 'static + Component>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.components_managers.contains_key(&type_id)
    }

    pub fn borrow_components<T: 'static + Component>(&self) -> &Vec<T> {
        self.borrow_component_manager::<T>().borrow_components()
    }

    pub fn borrow_components_mut<T: 'static + Component>(&mut self) -> &mut Vec<T> {
        self.borrow_component_manager_mut::<T>()
            .borrow_components_mut()
    }

    pub fn query_entities<T: 'static + Component>(&self) -> Option<Vec<usize>> {
        if !self.has_component_manager::<T>() {
            return None;
        }

        let component_query = self.query_manager.get_bit_for_component::<T>().unwrap();

        self.query_manager.query(*component_query)
    }

    pub fn borrow_components_for_entity<T: 'static + Component>(
        &mut self,
        entity: usize,
    ) -> Option<&mut T> {
        if !self.has_component_manager::<T>() {
            return None;
        }

        let type_id = TypeId::of::<T>();

        let manager = cast_manager_mut_unsafe::<T>(self.components_managers.get(&type_id).unwrap());
        manager.borrow_component_mut(entity)
    }

    pub fn query_entities_pair<T: 'static + Component, U: 'static + Component>(
        &self,
    ) -> Option<Vec<usize>> {
        if !self.has_component_manager::<T>() || !self.has_component_manager::<U>() {
            return None;
        }

        let component_query_t = self.query_manager.get_bit_for_component::<T>().unwrap();
        let component_query_u = self.query_manager.get_bit_for_component::<U>().unwrap();

        let query_bitmask = component_query_t | component_query_u;

        self.query_manager.query(query_bitmask)
    }

    pub fn borrow_components_pair_for_entity<T: 'static + Component, U: 'static + Component>(
        &mut self,
        entity: usize,
    ) -> Option<(&mut T, &mut U)> {
        if !self.has_component_manager::<T>() || !self.has_component_manager::<U>() {
            return None;
        }

        let type_id_t = TypeId::of::<T>();
        let type_id_u = TypeId::of::<U>();

        let manager_t =
            cast_manager_mut_unsafe::<T>(self.components_managers.get(&type_id_t).unwrap());
        let manager_u =
            cast_manager_mut_unsafe::<U>(self.components_managers.get(&type_id_u).unwrap());

        let component_t = manager_t.borrow_component_mut(entity).unwrap();
        let component_u = manager_u.borrow_component_mut(entity).unwrap();

        Some((component_t, component_u))
    }

    fn borrow_component_manager<T: 'static + Component>(&self) -> &ComponentManager<T> {
        let type_id = TypeId::of::<T>();
        cast_manager(self.components_managers.get(&type_id).unwrap().as_ref()).unwrap()
    }

    fn borrow_component_manager_mut<T: 'static + Component>(&mut self) -> &mut ComponentManager<T> {
        let type_id = TypeId::of::<T>();
        cast_manager_mut(self.components_managers.get_mut(&type_id).unwrap().as_mut()).unwrap()
    }
}

// This struct is used to manage the entities.
// IDs are reused when an entity is removed.
struct Entities {
    entities: Vec<Entity>,
    available_ids: Vec<usize>,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            available_ids: Vec::new(),
        }
    }

    pub fn has(&self, entity_id: usize) -> bool {
        entity_id < self.entities.len() && self.entities[entity_id].is_alive()
    }

    pub fn create(&mut self) -> usize {
        if self.available_ids.len() > 0 {
            let index = self.available_ids.pop().unwrap();
            self.entities[index].reset();
            return index;
        }

        let entity = Entity::new();
        self.entities.push(entity);

        self.entities.len() - 1
    }

    pub fn remove(&mut self, entity_id: usize) {
        if !self.has(entity_id) {
            return;
        }

        self.entities[entity_id].kill();
        self.available_ids.push(entity_id);
    }
}

fn cast_manager_mut_unsafe<T: 'static + Component>(
    manager: &Box<dyn ComponentManagerTrait>,
) -> &mut ComponentManager<T> {
    let ptr = cast_manager(manager.as_ref()).unwrap() as *const ComponentManager<T>
        as *mut ComponentManager<T>;
    unsafe { transmute(ptr) }
}
