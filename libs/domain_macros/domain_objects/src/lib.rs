/// ドメイン駆動設計（DDD）における「エンティティ」を表すトレイトです。
///
/// エンティティとは、属性の値ではなく、一意の識別子（Identity）によって定義されるドメインオブジェクトです。
/// 属性（名前や状態など）が変化しても、識別子が同じであれば同一のオブジェクトとみなされます。
/// 逆に、全ての属性が同じ値であっても、識別子が異なれば別のオブジェクトとして扱われます。
///
/// # 識別子による等価性（Identity-based Equality）
///
/// このトレイトを実装すると、オブジェクトの等価性判定（`PartialEq` および `Eq`）は
/// `identity()` メソッドが返す識別子の比較のみに基づいて行われるようになります。
/// これにより、構造体の全てのフィールドを比較するデフォルトの挙動ではなく、DDDの原則に従った比較が可能になります。
///
/// # Derive マクロによる利用
///
/// `#[derive(Entity)]` を使用することで、このトレイトの実装と、それに伴う `PartialEq`, `Eq` の実装を自動生成できます。
/// このとき、`PartialEq` や `Eq` を構造体側で別途 `derive` しないでください。`Entity` の派生マクロがこれらを実装するため、
/// 重複した実装指定はコンパイルエラーや意図しない動作の原因となります。識別子となるフィールドには `#[entity_id]` 属性を付与してください。
///
/// ## 例: 単一の識別子を持つエンティティ
///
/// ```rust
/// use domain_objects::EntityTrait; // 適切なパスに変更してください
/// use derive_entity::Entity;
///
/// #[derive(Debug, Entity)]
/// struct User {
///     #[entity_id]
///     id: u64,
///     name: String,
///     email: String,
/// }
///
/// let user1 = User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() };
/// let user2 = User { id: 1, name: "Alice (Updated)".to_string(), email: "new@example.com".to_string() };
/// let user3 = User { id: 2, name: "Alice".to_string(), email: "alice@example.com".to_string() };
///
/// // 識別子(id)が同じなので、他のフィールドが異なっていても等価とみなされる
/// assert_eq!(user1, user2);
///
/// // 内容が似ていても、識別子が異なるので別のエンティティとみなされる
/// assert_ne!(user1, user3);
/// ```
///
/// ## 例: 複合キーを持つエンティティ
///
/// 複数のフィールドに `#[entity_id]` を付与することで、複合キーを識別子として扱うことができます。
///
/// ```rust
/// use derive_entity::Entity;
///
/// #[derive(Entity)]
/// struct OrderItem {
///     #[entity_id]
///     order_id: u64,
///     #[entity_id]
///     item_id: u64,
///     quantity: u32,
/// }
/// ```
pub trait EntityTrait {
    /// 識別子を表す型です。
    /// ライフタイムを持つことで、フィールドへの参照をコストなく扱うことができます。
    type Identity<'a>
    where
        Self: 'a;

    /// オブジェクトの識別子（Identity）への参照を返します。
    /// 複合キーの場合は、フィールドへの参照を含むタプルが返されます。
    fn identity(&self) -> Self::Identity<'_>;

    /// 識別子に基づいて等価性を判定します。
    /// `derive(Entity)` マクロによって生成される `PartialEq` 実装は、内部的にこのメソッドを呼び出します。
    fn eq<'b>(&'b self, other: &'b Self) -> bool
    where
        Self::Identity<'b>: Eq,
    {
        self.identity() == other.identity()
    }
}
