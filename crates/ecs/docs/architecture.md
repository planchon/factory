# How it works ?
Multiple component in this packages : 

- EntityManager
- ComponentManager
- QueryManager

## ComponentManager 
The component manager helps store each component instance into an optimized maner.

## QueryManager 
The query manager helps find the right `entities` for a `query`. The query is generated from the `typeid` of the `Component` provided in the code.

```rust
let query: Option<Vec<usize>> = entity_manager.query_entities_pair::<Component1, Component2>();
```

## EntityManager
Entity manager binds the components manager and the query manager. When a system needs to get the entities it has access to, it will call the entity manager, which will performs the query. Then the system will get mutable references to components.