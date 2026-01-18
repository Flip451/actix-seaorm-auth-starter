use std::sync::Arc;

use crate::shared::outbox_event::OutboxRepository;

use super::user::UserRepository;
// use super::post::PostRepository; // 仮定: 追加されたリポジトリ

pub trait RepositoryFactory<'a>: Send + Sync {
    // 戻り値を Box にすることで、実体を持たずにトレイトオブジェクトとして扱います
    fn user_repository(&self) -> Arc<dyn UserRepository + 'a>;

    fn outbox_repository(&self) -> Arc<dyn OutboxRepository + 'a>;

    // 将来的な拡張:
    // fn post_repository(&self) -> Box<dyn PostRepository + '_>;
}
