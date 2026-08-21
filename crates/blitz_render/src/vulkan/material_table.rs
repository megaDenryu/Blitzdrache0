//! 索引化した材質テクスチャ表の台帳と世代。担当するのは、材質とテクスチャの安定IDを1つの資源表世代の中のGPU添字へ写す
//! 唯一の経路と、完全に構築済みの世代だけを公開してフェンス通過後に退役させる寿命の規律である。
//!
//! 束の登録から公開までの流れは次のとおりである。束が読み込まれると`材質登録簿`が材質とテクスチャへ安定IDを発番して
//! CPU側の材質を保持し、その全体から新しい世代を構築して公開する。フレームは公開中の世代を束縛し、旧世代は
//! 束縛した全フレームのフェンス通過後に退役する。起動時だけは公開する世代がまだ無いため、`材質の登録状態`へ
//! 起動シーンの束を積み終えてから最初の世代を構築し、それが通ったときだけ`材質資源表`を作る。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」

mod capacity;
mod device_supplier;
mod dry_run;
mod fallback_usage;
mod feature_set;
mod frame_hold;
mod generation;
mod generation_build;
mod generation_id;
mod generation_record;
mod generation_resolution;
mod image_id;
mod image_identity;
mod ledger;
mod material_gpu_reference;
mod material_id;
mod pack_input;
mod packer;
mod record_index;
mod registration;
mod registration_state;
mod residency_count;
mod resource_table;
mod stage_reserve;
mod supplier;
#[cfg(test)]
mod tests;
mod texture_id;
mod texture_registry;
mod texture_role;
mod texture_slot;
mod texture_spec;

/// レンダラーが持つ所有者と、GPU境界・描画経路が名指しする型だけを外へ見せる。
pub(crate) use capacity::テクスチャ表レイアウト容量;
pub(crate) use feature_set::材質特徴集合;
pub(crate) use generation_record::世代内材質レコード;
pub(crate) use generation_resolution::世代内材質解決;
pub(crate) use material_id::大域材質ID;
pub(crate) use registration_state::{描画対象別の材質ID, 材質の登録状態};
pub(crate) use resource_table::{材質資源の作業環境, 材質資源表, 資源表世代の束縛};
/// 役割の並びを直に組み立てるのは材質変種の検査だけであり、本番は`材質テクスチャ役割::全役割`を走査する。
#[cfg(test)]
pub(crate) use texture_role::役割の数;
pub(crate) use texture_role::材質テクスチャ役割;
