//! 編集資源と操作コマンドの型契約。ts-rsの生成元であり、検証をメソッドで持つ。
//! 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「層の定義」サーバー側の型契約層。

mod building;
mod building_definition_id;
mod building_grid;
mod building_outline_catalog;
mod chunk_coordinate;
mod chunk_road;
mod chunk_structure;
mod chunk_structure_version;
mod command;
mod grid;
mod material_board;
mod material_definition;
mod material_layer_assignment;
mod music;
mod numeric_check;
mod position;
mod regional_road;
mod scatter_settings;
mod scattered_individual;
mod text_check;
mod validation_error;
mod world_layout;
mod world_structure;
mod world_structure_version;

pub use building::建物の配置;
pub use building_definition_id::建物定義ID;
pub use building_grid::{
    はめ口の値, 升目の宣言, 升目の屋根, 升目の床, 升目の座標, 升目の複体, 壁の外面の飾り, 壁の種類, 建物の格子, 建物の格子の一覧項目,
    建物の格子の現在の形式版, 建物の格子の装飾,
};
pub use building_outline_catalog::{
    ベイ構造, 建物の入口方向, 建物の外接箱, 建物外形カタログ, 建物外形カタログの現在の形式版, 建物外形カタログ読み込みエラー, 建物外形定義,
    建物定義の用途,
};
pub use chunk_coordinate::チャンク座標;
pub use chunk_road::チャンクの道路;
pub use chunk_structure::チャンク構造;
pub use chunk_structure_version::チャンク構造移行エラー;
pub(crate) use chunk_structure_version::読み込んだチャンク構造の版;
pub use command::{
    地表材質層, 建物を削除する, 建物を移動する, 建物を配置する, 建物基礎を平坦化する, 急勾配を岩肌へベイクする, 散布設定を変更する, 材質の筆致,
    編集コマンド, 造成筆致, 造成筆致種別, 道路に合わせて切土盛土する, 道路を削除する, 道路を追加する, 道路下を泥へベイクする, 道路対象,
    道路点を削除する, 道路点を挿入する, 道路点を移動する, 道路点を追加する,
};
pub use grid::{チャンクの高さ編集, マザーハイトマップ, 地表材質の重み};
pub use material_board::マテリアル台帳;
pub use material_definition::マテリアル定義;
pub use material_layer_assignment::層割当;
pub use music::楽曲の版の移行エラー;
pub(crate) use music::読み込んだ楽曲の版;
pub use music::{
    コード進行, コード進行参照, トラックの格子, トラックの種類, トラック定義, パターン, パターンID, パターンのステップ数, ミキサー設定, 和音,
    和音の種類, 打楽器の種類, 既定のコード進行, 既定のコード進行一覧, 曲の節, 楽器, 楽曲, 楽曲ID, 楽曲の現在の形式版, 音の並び,
};
pub use position::位置3次元;
pub use regional_road::広域道路;
pub use scatter_settings::散布の設定;
pub use scattered_individual::散布の個体;
pub use validation_error::資源検証エラー;
pub use world_layout::世界の区画割り;
pub use world_structure::大域世界構造;
pub use world_structure_version::読み込んだ大域世界構造の版;
