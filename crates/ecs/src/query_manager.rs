use std::any::TypeId;
use std::collections::HashMap;

pub struct QueryManager {
    /// An entity is represented by a bitmask of components
    /// The first u128 is the bitmask of the components that the entity has
    /// The second u128 is the ID of the entity
    /// if bit_query & bit_entity != 0, then the entity matches the query
    query_entities: Vec<(u128, Vec<usize>)>,
    entities_query: HashMap<usize, u128>,
    bit_mapping: HashMap<TypeId, u128>,
    reusable_bits: Vec<u128>,
    next_bit: u128,
    /// The query cache is a map of bitmask to the entities that match the query
    /// The value is None if the query is not cached, otherwise it is the entities that match the query
    query_cache: HashMap<u128, Option<Vec<usize>>>,
}

impl QueryManager {
    pub fn new() -> Self {
        Self {
            query_entities: Vec::new(),
            entities_query: HashMap::new(),
            bit_mapping: HashMap::new(),
            next_bit: 1,
            reusable_bits: Vec::new(),
            query_cache: HashMap::new(),
        }
    }

    /// Register a component
    /// This tries to reuse a bit if possible, otherwise it will allocate a new one
    /// (at most 128 bits are allocated TODO: handle this)
    /// Will return the bit for the component
    pub fn register_component<T: 'static>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<T>();

        let bit = if self.reusable_bits.len() > 0 {
            self.reusable_bits.remove(0)
        } else {
            let old_next_bit = self.next_bit;
            self.next_bit *= 2;
            old_next_bit
        };

        self.bit_mapping.insert(type_id, bit);
        self
    }

    /// Unregister a component
    /// Will panic if the component is not registered
    pub fn unregister_component<T: 'static>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<T>();
        let bit = self.bit_mapping.remove(&type_id).unwrap();
        self.reusable_bits.push(bit);
        self
    }

    /// Get the bits for a component
    /// Will panic if the component is not registered
    pub fn get_bit_for_component<T: 'static>(&self) -> Option<&u128> {
        let type_id = TypeId::of::<T>();
        self.bit_mapping.get(&type_id)
    }

    pub fn get_bitmask_for_entity(&self, entity_id: usize) -> u128 {
        let bitmask = self.entities_query.get(&entity_id);
        if let Some(bitmask) = bitmask {
            return bitmask.clone();
        }
        0
    }

    pub fn remove_entity(&mut self, entity_id: usize) -> &mut Self {
        let index = self
            .query_entities
            .iter()
            .position(|(_, ids)| ids.contains(&entity_id));
        if let Some(index) = index {
            self.query_entities[index].1.retain(|id| *id != entity_id);
        }

        self.entities_query.remove(&entity_id);

        self
    }

    pub fn add_entity(&mut self, entity_id: usize, entity_bitmask: u128) -> &mut Self {
        let index = self
            .query_entities
            .iter()
            .position(|(bitmask, _)| *bitmask == entity_bitmask);
        if let Some(index) = index {
            self.query_entities[index].1.push(entity_id);
        } else {
            self.query_entities.push((entity_bitmask, vec![entity_id]));
        }

        self.entities_query.insert(entity_id, entity_bitmask);

        self
    }

    /// Query the entities that match the bitmask
    pub fn query(&self, query_bitmask: u128) -> Option<Vec<usize>> {
        // if self.query_cache.contains_key(&bitmask) {
        //     return self.query_cache.get(&bitmask).unwrap().clone();
        // }

        let entities = self
            .query_entities
            .iter()
            .filter(|(bitmask, _)| *bitmask & query_bitmask == query_bitmask)
            .map(|(_, ids)| ids.clone())
            .flatten()
            .collect();

        // self.query_cache.insert(bitmask, Some(entities.clone()));

        Some(entities)
    }
}
