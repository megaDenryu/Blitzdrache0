//! エンジン層: ゲームロジックに描画対象の世界を表現する語彙を提供し、blitz_renderのレンダーグラフへ描画内容を翻訳する。注意: このクレートはwinitにもashにも依存せず、入力はデバイス非依存の`カメラインテント`としてのみ受け取る（参照: `_doc/計画/ユビキタス言語.md`「入力インテント」）。
#![forbid(unsafe_code)]
// `pub mod`で公開する語彙は、いずれも型と定数の数が多く、クレート直下の平坦な再エクスポートへ混ぜると
// 呼び出し側が出どころを追えなくなるものである。そのためモジュールごと公開する。
pub mod animation;
mod asset;
pub mod auto_exposure;
mod camera;
mod chunk;
mod frame_composition;
pub mod height_field;
mod instance_lod;
#[cfg(test)]
mod instance_lod_tests;
mod lighting;
pub mod local_visibility;
mod lod_threshold;
pub mod primitive_draw_item;
pub mod sky;
pub mod static_shape;
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
mod streaming_directory_replace_tests;
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
pub mod surface_layer_textures;
pub mod temporal_reconstruction;
mod terrain_lod;
#[cfg(test)]
mod terrain_lod_tests;
pub mod time;
mod visibility;
#[cfg(test)]
mod visibility_tests;
mod world_query;
pub mod world_shape_query;
pub use asset::{
    texture_storage, アセットID, アセットIDエラー, アセットメタデータ, アセット実行時形式エラー, アセット形式版, アニメーションクリップ,
    インスタンス群, インスタンス群エラー, カタログ, カタログを実行時形式へ格納する, カタログ項目, シーンを実行時形式へ格納する, シーンデータ,
    ジョイント, ジョイントアニメーションチャンネル, スキンデータ, スキン頂点属性, チャンクの基準原点からの許容メートル,
    チャンク目録を実行時形式へ格納する, チャンネル, テクスチャデータ, マテリアルデータ, メッシュデータ, メッシュプリミティブ, メッシュ頂点属性,
    世界の乱数の種, 世界の生成器の版, 世界の由来, 個体配置, 原型, 地形LODメッシュ群, 地形LODメッシュ群エラー, 地表の層の重ね合わせデータ,
    地表の層の重ね合わせデータエラー, 境界球, 実行時アセット, 実行時アセットの公開完了印, 実行時アセットを格納する, 実行時アセットを開く,
    実行時アセット種別, 実行時カタログのファイル, 実行時カタログ読込エラー, 実行時シーンのファイル, 実行時シーンの内容, 実行時シーン読込エラー,
    実行時形式からカタログを読む, 実行時形式からシーンを読む, 実行時形式からチャンク目録を読む, 実行時形式のファイルの読み取りの破れ, 描画対象ID,
    描画対象データ, 描画形状, 材質スロットID, 材質スロット割当, 材質割当エラー, 材質特徴集合, 材質集合, 材質集合エラー, 群境界, 補間種別,
    軸平行包囲領域, 金属粗さPBRデータ, 静的TRS, 高さ場の標本数,
};
pub use camera::{カメラ, カメラインテント};
pub use chunk::チャンク座標;
pub use frame_composition::既定フレーム構成を作る;
pub use instance_lod::{個体LODエラー, 個体LOD選択設定, 個体別段状態, 個体詳細段};
pub use lighting::{天空状態をライティングへ写す, 既定の影響半径を作る, 既定ライティングを作る};
pub use streaming::{
    GPU転送完了結果, ストリーミングメモリ量, ストリーミング予算, ストリーミング予算エラー, ストリーミング予算結果, ストリーミング見積エラー,
    ストリーミング調停, ストリーミング調停エラー, ストリーミング調停設定, ストリーミング転送量, ストリーミング進行, チャンクメモリ量を見積もる,
    チャンク一辺, チャンク一辺エラー, チャンク予算候補, チャンク台帳, チャンク台帳エラー, チャンク格子, チャンク格子エラー, チャンク状態,
    チャンク目録, チャンク目録エラー, チャンク目録差し替えエラー, チャンク要求, チャンク読込エラー, チャンク読込器, チャンク読込完了,
    チャンク読込成果, チャンク読込設定, チャンク読込設定エラー, チャンク集合差分, リセット世代, 予算判定, 固定経路, 固定経路エラー,
    実行時チャンク目録のファイル, 実行時チャンク目録読込エラー, 準備完了結果, 観測済み読込量, 解除報告結果, 退避結果,
};
pub use terrain_lod::{地形LODエラー, 地形LOD選択器, 地形LOD選択設定};
pub use visibility::{
    キャスター距離分布, 個体境界球, 個体選択材料, 可視個体と段の選別係, 可視判定エラー, 可視判定方式, 可視判定計数, 影の視距離, 段選択方式,
    群可視材料, 視錐台, 距離区分別の光空間直方体, 距離区分別の可視判定,
};
pub use world_query::{世界問い合わせ結果, 問い合わせ完全性};
