use crate::domain::user::UserRepository;
// use crate::domain::post::PostRepository; // 仮定: 追加されたリポジトリ

// トランザクション内で利用可能なリポジトリ一覧を保持する構造体
pub struct TxRepositories<'a> {
    pub user: &'a dyn UserRepository,
    // pub post: Arc<dyn PostRepository>, // 追加されたらここに増やす
}
