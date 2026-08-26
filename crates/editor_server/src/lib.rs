//! 編集サーバーのクレートルート。コンポジションルートは`main.rs`が担う。
//! 統合テストがルーターを直接組み立てて使えるよう、状態とルーター構築だけを公開する。
#![forbid(unsafe_code)]

mod atomic_file_write;
mod bake_editor_world;
mod building_grid_store;
mod chunk_height_supply;
#[cfg(feature = "typescript")]
mod contract;
mod export;
mod failure_response;
mod health_contract;
mod mother_height_cutout;
mod normalized_app;
mod project_info_contract;
mod project_root;
mod repository_root;
mod resource;
mod routes;
mod server_state;
mod storage;

pub use building_grid_store::{建物の格子の保存エラー, 建物の格子の保存係};
#[cfg(feature = "typescript")]
pub use contract::{契約ファイルの本文を組み立てる, 編集資源契約の本文を組み立てる};
pub use export::{
    ソースアセット書き出しコマンド, 一棟だけの検証世界を書き出すコマンド, 世界ソース出力先, 出力世界名, 書き出しエラー, 書き出し結果
};
pub use health_contract::生存確認応答;
pub use mother_height_cutout::高さの切り出しエラー;
pub use normalized_app::経路正規化アプリ;
pub use project_info_contract::プロジェクト情報応答;
pub use project_root::{プロジェクトルート, プロジェクトルートを解決する};
pub use repository_root::リポジトリルート;
pub use resource::{
    はめ口の値, コード進行, コード進行参照, チャンクの道路, チャンクの高さ編集, チャンク座標, チャンク構造, トラックの格子, トラックの楽器を変える,
    トラックの種類, トラックの進行の割り当てを変える, トラックの音量を変える, トラック定義, パターン, パターンID, パターンのステップ数,
    パターンの打点を全部消す, パターンの表示名を変える, パターンの進行を変える, パターンを削除する, パターンを追加する, ベイ構造, マザーハイトマップ,
    マテリアル台帳, マテリアル定義, ミキサー設定, ミキサー設定を変える, 世界の区画割り, 位置3次元, 升目の宣言, 升目の屋根, 升目の床, 升目の座標,
    升目の複体, 和音, 和音の種類, 地表材質の重み, 地表材質層, 壁の外面の飾り, 壁の種類, 大域世界構造, 層割当, 広域道路, 建物の入口方向, 建物の外接箱,
    建物の格子, 建物の格子の一覧項目, 建物の格子の現在の形式版, 建物の格子の装飾, 建物の配置, 建物を削除する, 建物を移動する, 建物を配置する,
    建物基礎を平坦化する, 建物外形カタログ, 建物外形カタログの現在の形式版, 建物外形カタログ読み込みエラー, 建物外形定義, 建物定義ID, 建物定義の用途,
    急勾配を岩肌へベイクする, 打ち込みの対象, 打楽器の種類, 打点を消す, 打点を置く, 拍毎分を変える, 散布の個体, 散布の設定, 散布設定を変更する,
    既定のコード進行, 既定のコード進行一覧, 曲の節, 曲の節を並べ替える, 曲の節を削除する, 曲の節を変える, 曲の節を追加する, 材質の筆致, 楽器, 楽曲,
    楽曲ID, 楽曲の版の移行エラー, 楽曲の現在の形式版, 楽曲の表示名を変える, 楽曲編集コマンド, 独自の進行を保存する, 独自の進行を削除する,
    範囲の打点を消す, 編集コマンド, 読み込んだ大域世界構造の版, 造成筆致, 造成筆致種別, 道路に合わせて切土盛土する, 道路を削除する, 道路を追加する,
    道路下を泥へベイクする, 道路対象, 道路点を削除する, 道路点を挿入する, 道路点を移動する, 道路点を追加する, 音の並び, 音を伸ばす,
};
pub use routes::ルーターを組み立てる;
pub use server_state::サーバー状態;
pub use storage::{ファイル保管庫, プロジェクト保管庫};
