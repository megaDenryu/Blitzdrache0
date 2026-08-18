//! 編集サーバーのクレートルート。コンポジションルートは`main.rs`が担う。
//! 統合テストがルーターを直接組み立てて使えるよう、状態とルーター構築だけを公開する。
#![forbid(unsafe_code)]

#[cfg(feature = "typescript")]
mod contract;
mod failure_response;
mod health_contract;
mod normalized_app;
mod repository_root;
mod routes;
mod server_state;

#[cfg(feature = "typescript")]
pub use contract::契約ファイルの本文を組み立てる;
pub use health_contract::生存確認応答;
pub use normalized_app::経路正規化アプリ;
pub use repository_root::リポジトリルートを解決する;
pub use routes::ルーターを組み立てる;
pub use server_state::サーバー状態;
