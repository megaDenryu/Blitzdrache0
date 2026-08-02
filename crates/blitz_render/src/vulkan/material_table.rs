//! 索引化した材質テクスチャ表の台帳と世代。担当するのは、材質とテクスチャの安定IDを1つの資源表世代の中のGPU添字へ写す
//! 唯一の経路と、完全に構築済みの世代だけを公開してフェンス通過後に退役させる寿命の規律である。
//!
//! 注意: 段4aの契約により、この表を束縛するディスクリプタセットもシェーダーもまだ無い。現行の描画は旧スロット別セット経路の
//! ままであり、この表の内容は絵に影響しない。フレームごとの束縛(`フレームが束縛する`)と退役の回収の呼び出し元は
//! 段4bのフレーム経路が持つ。そのため`vulkan/mod.rs`のモジュール宣言がこの木の未使用を許している。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」

mod capacity;
mod device_supplier;
mod fallback_usage;
mod feature_set;
mod frame_hold;
mod generation;
mod generation_build;
mod generation_id;
mod generation_record;
mod image_id;
mod image_identity;
mod ledger;
mod material_gpu_reference;
mod material_id;
mod pack_input;
mod packer;
mod record_index;
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

/// レンダラーが持つ所有者だけを外へ見せる。他の型はこのモジュール木の内側だけで使う。
pub(crate) use resource_table::材質資源表;
