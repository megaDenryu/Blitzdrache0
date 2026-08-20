//! 編集コマンドの型契約。判断1(操作コマンドはブラウザとヘッドレスの2つの入り口から、
//! 同じTS実装で適用する)に基づき、ここは直列化形のみを定義する。適用実装(取り消し・
//! やり直しを含む)はTS側の編集モデルが持ち、段2では作らない。

mod building_ops;
mod material_stroke;
mod road_collection_ops;
mod road_point_ops;
mod road_target;
mod scatter_settings_change;
mod sculpt_stroke;
mod terrain_bake_ops;

use serde::{Deserialize, Serialize};

pub use building_ops::{建物を削除する, 建物を移動する, 建物を配置する};
pub use material_stroke::{地表材質層, 材質の筆致};
pub use road_collection_ops::{道路を削除する, 道路を追加する};
pub use road_point_ops::{道路点を削除する, 道路点を挿入する, 道路点を移動する, 道路点を追加する};
pub use road_target::道路対象;
pub use scatter_settings_change::散布設定を変更する;
pub use sculpt_stroke::{造成筆致, 造成筆致種別};
pub use terrain_bake_ops::{
    建物基礎を平坦化する, 急勾配を岩肌へベイクする, 道路に合わせて切土盛土する, 道路下を泥へベイクする
};

/// 編集コマンドとは、人間の画面操作とAIのヘッドレス操作の両方が同じ形で積む、
/// 1回ぶんの編集操作を表す判別共用体のことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[serde(tag = "種類", content = "値")]
pub enum 編集コマンド {
    造成筆致(造成筆致),
    材質の筆致(材質の筆致),
    道路を追加する(道路を追加する),
    道路を削除する(道路を削除する),
    道路点を追加する(道路点を追加する),
    道路点を挿入する(道路点を挿入する),
    道路点を移動する(道路点を移動する),
    道路点を削除する(道路点を削除する),
    建物を配置する(建物を配置する),
    建物を移動する(建物を移動する),
    建物を削除する(建物を削除する),
    散布設定を変更する(散布設定を変更する),
    道路に合わせて切土盛土する(道路に合わせて切土盛土する),
    建物基礎を平坦化する(建物基礎を平坦化する),
    急勾配を岩肌へベイクする(急勾配を岩肌へベイクする),
    道路下を泥へベイクする(道路下を泥へベイクする),
}
