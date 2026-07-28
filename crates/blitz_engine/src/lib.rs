//! エンジン層: シーン・アセット・マテリアル。
//!
//! 責務: ゲームロジックに対して描画対象の世界を表現する語彙を提供し、
//! blitz_render のレンダーグラフへ描画内容を翻訳する。
//!
//! 注意: このクレートはwinitにもashにも依存しない。入力はデバイス非依存の
//! `カメラインテント` としてのみ受け取る（参照: `_doc/計画/ユビキタス言語.md`「入力インテント」）。

#![forbid(unsafe_code)]

mod animation;
mod asset;
mod camera;
mod chunk;
mod frame_composition;
#[cfg(test)]
mod frame_composition_tests;
mod instance_lod;
#[cfg(test)]
mod instance_lod_tests;
mod lighting;
mod lod_threshold;
mod streaming;
#[cfg(test)]
mod streaming_budget_overflow_tests;
#[cfg(test)]
mod streaming_budget_tests;
#[cfg(test)]
mod streaming_chunk_world_fixture;
#[cfg(test)]
mod streaming_coordinator_fixture;
#[cfg(test)]
mod streaming_coordinator_tests;
#[cfg(test)]
mod streaming_directory_tests;
#[cfg(test)]
mod streaming_estimate_tests;
#[cfg(test)]
mod streaming_eviction_tests;
#[cfg(test)]
mod streaming_fixed_route_tests;
#[cfg(test)]
mod streaming_ledger_cancel_tests;
#[cfg(test)]
mod streaming_ledger_tests;
#[cfg(test)]
mod streaming_loader_tests;
#[cfg(test)]
mod streaming_observed_read_size_tests;
#[cfg(test)]
mod streaming_priority_eviction_tests;
#[cfg(test)]
mod streaming_reset_generation_tests;
#[cfg(test)]
mod streaming_reset_tests;
#[cfg(test)]
mod streaming_tests;
#[cfg(test)]
mod streaming_transfer_tests;
#[cfg(test)]
mod streaming_usage_tests;
mod terrain_lod;
#[cfg(test)]
mod terrain_lod_tests;
mod visibility;
#[cfg(test)]
mod visibility_tests;

pub use animation::{スキン行列を計算する, スキン行列一覧, ブレンドする, 姿勢, 姿勢を評価する, 関節TRS};
pub use asset::{
    アセットID, アセットIDエラー, アセットメタデータ, アセット実行時形式エラー, アセット形式版, アニメーションクリップ, インスタンス群,
    インスタンス群エラー, カタログ, カタログを実行時形式へ格納する, カタログ項目, シーンを実行時形式へ格納する, シーンデータ, ジョイント,
    ジョイントアニメーションチャンネル, スキンデータ, スキン頂点属性, チャンクアンカーからの許容メートル, チャンク目録を実行時形式へ格納する,
    チャンネル, テクスチャデータ, マテリアルデータ, メッシュデータ, メッシュ頂点属性, 個体配置, 原型, 地形LODメッシュ群, 地形LODメッシュ群エラー,
    境界球, 実行時アセット, 実行時アセットを格納する, 実行時アセットを開く, 実行時アセット種別, 実行時カタログを読み込む, 実行時カタログ読込エラー,
    実行時シーンを読み込む, 実行時シーン読込エラー, 実行時形式からカタログを読む, 実行時形式からシーンを読む, 実行時形式からチャンク目録を読む,
    描画対象ID, 描画対象データ, 描画形状, 法線マップ既定テクスチャを作る, 白テクスチャデータを作る, 群境界, 補間種別, 軸平行包囲領域,
    金属粗さPBRデータ, 静的TRS,
};
pub use camera::{カメラ, カメラインテント};
pub use chunk::チャンク座標;
pub use frame_composition::既定フレーム構成を作る;
pub use instance_lod::{個体LODエラー, 個体LOD選択設定, 個体別段状態, 個体詳細段};
pub use lighting::既定ライティングを作る;
pub use streaming::{
    GPU転送完了結果, ストリーミングメモリ量, ストリーミング予算, ストリーミング予算エラー, ストリーミング予算結果, ストリーミング見積エラー,
    ストリーミング調停, ストリーミング調停エラー, ストリーミング調停設定, ストリーミング転送量, ストリーミング進行, チャンクメモリ量を見積もる,
    チャンク予算候補, チャンク台帳, チャンク台帳エラー, チャンク格子, チャンク格子エラー, チャンク状態, チャンク目録, チャンク目録エラー,
    チャンク要求, チャンク読込エラー, チャンク読込器, チャンク読込完了, チャンク読込成果, チャンク集合差分, リセット世代, 予算判定, 固定経路,
    固定経路エラー, 実行時チャンク目録を読み込む, 実行時チャンク目録読込エラー, 準備完了結果, 観測済み読込量, 解除報告結果, 退避結果,
};
pub use terrain_lod::{地形LODエラー, 地形LOD選択器, 地形LOD選択設定};
pub use visibility::{個体境界球, 可視個体を選ぶ, 可視判定エラー, 可視判定計数, 群可視材料, 視錐台};
