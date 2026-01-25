use domain_macros::Entity;

#[derive(Entity)]
struct User {
    id: i32, // 属性をつけ忘れた
    name: String,
}

fn main() {}