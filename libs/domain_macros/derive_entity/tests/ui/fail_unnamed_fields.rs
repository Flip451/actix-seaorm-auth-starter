use derive_entity::Entity;

#[derive(Entity)]
struct TupleUser(#[entity_id] i32, String);

fn main() {}