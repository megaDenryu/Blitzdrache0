//! 編集サーバーのクレートルート。コンポジションルートは`main.rs`が担う。
//! 統合テストがルーターを直接組み立てて使えるよう、状態とルーター構築だけを公開する。
#![forbid(unsafe_code)]

mod atomic_file_write;
#[cfg(feature = "typescript")]
mod contract;
mod export;
mod failure_response;
mod health_contract;
mod normalized_app;
mod project_info_contract;
mod project_root;
mod repository_root;
mod resource;
mod routes;
mod server_state;
mod storage;

#[cfg(feature = "typescript")]
pub use contract::{契約ファイルの本文を組み立てる, 編集資源契約の本文を組み立てる};
pub use export::{
    ソースアセット書き出しコマンド, 世界ソース出力先, 出力世界名, 書き出しエラー, 書き出し結果
};
pub use health_contract::生存確認応答;
pub use normalized_app::経路正規化アプリ;
pub use project_info_contract::プロジェクト情報応答;
pub use project_root::{プロジェクトルート, プロジェクトルートを解決する};
pub use repository_root::リポジトリルート;
pub use resource::{
    チャンクの道路, チャンクの高さ編集, チャンク座標, チャンク構造, マザーハイトマップ, 世界の区画割り, 位置3次元, 地表材質の重み, 地表材質層,
    大域世界構造, 広域道路, 建物の配置, 建物を削除する, 建物を移動する, 建物を配置する, 建物基礎を平坦化する, 建物種別, 急勾配を岩肌へベイクする,
    散布の設定, 散布設定を変更する, 材質の筆致, 編集コマンド, 造成筆致, 造成筆致種別, 道路に合わせて切土盛土する, 道路下を泥へベイクする, 道路対象,
    道路点を削除する, 道路点を移動する, 道路点を追加する,
};
pub use routes::ルーターを組み立てる;
pub use server_state::サーバー状態;
pub use storage::{ファイル保管庫, プロジェクト保管庫};
