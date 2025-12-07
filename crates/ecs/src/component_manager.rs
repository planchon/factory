use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::component::Component;

// store all the components T
pub struct ComponentManager<T: Component> {
    // all the components structures
    components: Vec<T>,
    // all the entities ids
    entities_ids: Vec<usize>,
    // map the entity id to the component index
    entity_to_component_index: HashMap<usize, usize>,
}

pub trait ComponentManagerTrait {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn has(&self, entity_id: usize) -> bool;
    fn remove(&mut self, entity_id: usize);
    fn get_type_id(&self) -> TypeId;
}

impl<T: 'static + Component> ComponentManagerTrait for ComponentManager<T> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn has(&self, entity_id: usize) -> bool {
        let manager = cast_manager::<T>(self).unwrap();
        manager.has(entity_id)
    }

    fn remove(&mut self, entity_id: usize) {
        let manager = cast_manager_mut::<T>(self).unwrap();
        manager.remove(entity_id)
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

pub fn cast_manager<T: 'static + Component>(
    manager: &dyn ComponentManagerTrait,
) -> Option<&ComponentManager<T>> {
    manager.as_any().downcast_ref::<ComponentManager<T>>()
}

pub fn cast_manager_mut<T: 'static + Component>(
    manager: &mut dyn ComponentManagerTrait,
) -> Option<&mut ComponentManager<T>> {
    manager.as_any_mut().downcast_mut::<ComponentManager<T>>()
}

impl<T: 'static + Component> ComponentManager<T> {
    pub fn new() -> Self {
        ComponentManager {
            components: Vec::new(),
            entities_ids: Vec::new(),
            entity_to_component_index: HashMap::new(),
        }
    }

    pub fn has(&self, entity_id: usize) -> bool {
        self.entity_to_component_index.contains_key(&entity_id)
    }

    pub fn add(&mut self, entity_id: usize, component: T) {
        if self.has(entity_id) {
            return;
        }

        self.components.push(component);
        self.entities_ids.push(entity_id);

        let component_index = self.components.len() - 1;
        self.entity_to_component_index
            .insert(entity_id, component_index);
    }

    pub fn remove(&mut self, entity_id: usize) {
        if !self.has(entity_id) {
            return;
        }

        let component_index = self.entity_to_component_index.remove(&entity_id).unwrap();
        // give component_index place to the last entity
        // that way we can use swap_remove to remove the last element
        self.entity_to_component_index
            .insert(*self.entities_ids.last().unwrap(), component_index);

        self.components.swap_remove(component_index);
        self.entities_ids.swap_remove(component_index);

        // remove the entity id from the map because it's not in the components anymore
        self.entity_to_component_index.remove(&entity_id);
    }

    pub fn borrow_component_for_entity(&self, entity_id: usize) -> Option<&T> {
        if !self.has(entity_id) {
            return None;
        }

        let component_index = self.entity_to_component_index.get(&entity_id).unwrap();
        Some(&self.components[*component_index])
    }

    pub fn borrow_component_mut(&mut self, entity_id: usize) -> Option<&mut T> {
        if !self.has(entity_id) {
            return None;
        }

        let component_index = self.entity_to_component_index.get(&entity_id).unwrap();
        Some(&mut self.components[*component_index])
    }

    pub fn borrow_components(&self) -> &Vec<T> {
        &self.components
    }

    pub fn borrow_components_mut(&mut self) -> &mut Vec<T> {
        &mut self.components
    }
}
