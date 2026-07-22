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
mod frame_composition;
#[cfg(test)]
mod frame_composition_tests;
mod lighting;
mod streaming;
#[cfg(test)]
mod streaming_ledger_cancel_tests;
#[cfg(test)]
mod streaming_ledger_tests;
#[cfg(test)]
mod streaming_loader_tests;
#[cfg(test)]
mod streaming_tests;

pub use animation::{スキン行列を計算する, スキン行列一覧, ブレンドする, 姿勢, 姿勢を評価する, 関節TRS};
pub use asset::{
    アセットID, アセットIDエラー, アセットメタデータ, アセット実行時形式エラー, アセット形式版, アニメーションクリップ, カタログ,
    カタログを実行時形式へ格納する, カタログ項目, シーンを実行時形式へ格納する, シーンデータ, ジョイント, ジョイントアニメーションチャンネル,
    スキンデータ, スキン頂点属性, チャンクID, チャンネル, テクスチャデータ, マテリアルデータ, メッシュデータ, メッシュ頂点属性, 実行時アセット,
    実行時アセットを格納する, 実行時アセットを開く, 実行時アセット種別, 実行時カタログを読み込む, 実行時カタログ読込エラー, 実行時シーンを読み込む,
    実行時シーン読込エラー, 実行時形式からカタログを読む, 実行時形式からシーンを読む, 描画対象ID, 描画対象データ, 法線マップ既定テクスチャを作る,
    白テクスチャデータを作る, 補間種別, 金属粗さPBRデータ, 静的TRS,
};
pub use camera::{カメラ, カメラインテント};
pub use frame_composition::既定フレーム構成を作る;
pub use lighting::既定ライティングを作る;
pub use streaming::{
    GPU転送完了結果, チャンク台帳, チャンク台帳エラー, チャンク座標, チャンク格子, チャンク格子エラー, チャンク状態, チャンク要求,
    チャンク読込エラー, チャンク読込器, チャンク読込完了, チャンク読込成果, チャンク集合差分, 準備完了結果,
};
