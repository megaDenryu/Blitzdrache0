//! 編集資源と操作コマンドの型契約。ts-rsの生成元であり、検証をメソッドで持つ。
//! 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「層の定義」サーバー側の型契約層。

mod building;
mod chunk_coordinate;
mod chunk_road;
mod chunk_structure;
mod command;
mod grid;
mod numeric_check;
mod position;
mod regional_road;
mod scatter_settings;
mod validation_error;
mod world_layout;
mod world_structure;

pub use building::{建物の配置, 建物種別};
pub use chunk_coordinate::チャンク座標;
pub use chunk_road::チャンクの道路;
pub use chunk_structure::チャンク構造;
pub use command::{
    地表材質層, 建物を削除する, 建物を移動する, 建物を配置する, 建物基礎を平坦化する, 急勾配を岩肌へベイクする, 散布設定を変更する, 材質の筆致,
    編集コマンド, 造成筆致, 造成筆致種別, 道路に合わせて切土盛土する, 道路下を泥へベイクする, 道路対象, 道路点を削除する, 道路点を挿入する,
    道路点を移動する, 道路点を追加する,
};
pub use grid::{チャンクの高さ編集, マザーハイトマップ, 地表材質の重み};
pub use position::位置3次元;
pub use regional_road::広域道路;
pub use scatter_settings::散布の設定;
pub use validation_error::資源検証エラー;
pub use world_layout::世界の区画割り;
pub use world_structure::大域世界構造;
