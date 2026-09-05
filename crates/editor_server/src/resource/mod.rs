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
mod coarse_cell_paint;
mod command;
mod contour_line;
mod grid;
mod material_board;
mod material_definition;
mod material_layer_assignment;
mod music;
mod numeric_check;
mod plan_view_draft;
mod plane_position;
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
pub use coarse_cell_paint::粗マスの塗り;
pub use command::{
    地表材質層, 建物を削除する, 建物を移動する, 建物を配置する, 建物基礎を平坦化する, 急勾配を岩肌へベイクする, 散布設定を変更する, 材質の筆致,
    等高線から高さ場を生成する, 等高線を削除する, 等高線を変更する, 等高線を追加する, 粗マスから地形を生成する, 粗マスを塗る, 編集コマンド, 造成筆致,
    造成筆致種別, 道路に合わせて切土盛土する, 道路を削除する, 道路を追加する, 道路下を泥へベイクする, 道路対象, 道路点を削除する, 道路点を挿入する,
    道路点を移動する, 道路点を追加する, 高さ場から等高線を導く, 高さ場から粗マスを導く,
};
pub use contour_line::等高線;
pub use grid::{チャンクの高さ編集, マザーハイトマップ, 地表材質の重み};
pub use material_board::マテリアル台帳;
pub use material_definition::マテリアル定義;
pub use material_layer_assignment::層割当;
pub use music::楽曲の版の移行エラー;
pub(crate) use music::読み込んだ楽曲の版;
pub use music::{
    コード進行, コード進行参照, テンポの上限, テンポの下限, テンポを変える, トラックの格子, トラックの楽器を変える, トラックの種類,
    トラックの進行の割り当てを変える, トラックの音量を変える, トラック定義, パターン, パターンID, パターンの小節数の上限, パターンの小節数を変える,
    パターンの打点を全部消す, パターンの表示名を変える, パターンの進行を変える, パターンを削除する, パターンを追加する, ミキサー設定,
    ミキサー設定を変える, 和音, 和音の根音の上限, 和音の根音の下限, 和音の種類, 和音の続くステップ数の上限, 和音の続くステップ数の下限,
    小節あたりのステップ数, 打ち込みの対象, 打楽器の種類, 打点を消す, 打点を置く, 新しいパターンの既定の小節数, 既定のコード進行,
    既定のコード進行一覧, 曲の節, 曲の節の繰り返し回数の上限, 曲の節の繰り返し回数の下限, 曲の節を並べ替える, 曲の節を削除する, 曲の節を変える,
    曲の節を挿入する, 曲の節を追加する, 楽器, 楽曲, 楽曲ID, 楽曲の現在の形式版, 楽曲の表示名を変える, 楽曲編集コマンド, 独自の進行を保存する,
    独自の進行を削除する, 範囲の打点を消す, 遅延のステップ数の上限, 遅延のステップ数の下限, 音の並び, 音を伸ばす, 音量と効果の比の上限,
    音量と効果の比の下限, 音高番号の上限, 音高番号の下限,
};
pub use plan_view_draft::{既定の粗マスの一辺の升目数, 見下ろし図の下書き};
pub use plane_position::平面の位置;
pub use position::位置3次元;
pub use regional_road::広域道路;
pub use scatter_settings::散布の設定;
pub use scattered_individual::散布の個体;
pub use validation_error::資源検証エラー;
pub use world_layout::世界の区画割り;
pub use world_structure::大域世界構造;
pub use world_structure_version::読み込んだ大域世界構造の版;
