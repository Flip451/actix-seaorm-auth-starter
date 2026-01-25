use domain_macros::Entity;

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserId(i32);

#[test]
fn test_entity_equality_single_id() {
    #[derive(Entity, Debug)]
    struct User {
        #[entity_id]
        id: UserId,
        name: String,
        age: i32,
    }

    let user1 = User {
        id: UserId(1),
        name: "Alice".to_string(),
        age: 20,
    };

    // ID は同じだが、他のフィールドが異なるユーザー
    let user2 = User {
        id: UserId(1),
        name: "Bob".to_string(),
        age: 25,
    };

    // ID のみが異なるユーザー
    let user3 = User {
        id: UserId(2),
        name: "Alice".to_string(),
        age: 20,
    };

    // ID が同じ場合は等しいことを確認
    assert_eq!(user1, user2, "Users with the same ID should be equal");

    // ID が異なる場合は等しくないことを確認
    assert_ne!(user1, user3, "Users with different IDs should not be equal");
}

#[test]
fn test_entity_equality_composite_id() {
    #[derive(Entity, Debug)]
    struct OrderItem {
        #[entity_id]
        order_id: i32,
        #[entity_id]
        item_id: i32,
        quantity: i32,
    }

    let item1 = OrderItem {
        order_id: 10,
        item_id: 1,
        quantity: 5,
    };
    let item2 = OrderItem {
        order_id: 10,
        item_id: 1,
        quantity: 999,
    };
    let item3 = OrderItem {
        order_id: 10,
        item_id: 2,
        quantity: 5,
    };

    assert_eq!(item1, item2, "Composite ID match should imply equality");
    assert_ne!(
        item1, item3,
        "Partial match of composite ID should imply inequality"
    );
}
