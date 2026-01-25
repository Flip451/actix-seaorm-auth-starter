use domain_macros::Entity;

#[derive(Entity)]
struct TupleUser(#[entity_id] i32, String);

fn main() {}