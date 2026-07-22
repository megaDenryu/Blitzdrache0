//! レンダーグラフ + リソースシステム。エンジン中核の型安全層。
//!
//! 責務: Vulkan(ash) の unsafe をこのクレートの実装内部に封じ込め、
//! 上位層には「このエンジンの使い方」だけを表現した安全な型を公開する。
//!
//! 注意: 公開APIに ash の型（vk::* やハンドル）を露出してはならない。
//! 露出した瞬間、unsafe の封じ込めが破れて上位層に責任が漏れる。

mod clear_color;
mod cloth_material;
mod cloth_shader_set;
mod compute_shader;
mod draw_result;
mod error;
mod extent;
mod frame_input;
mod gpu_memory_stats;
mod material;
mod particle_material;
mod particle_shader_set;
mod pbr_material;
mod readback_image;
mod readback_result;
mod render_object_material;
mod render_scene_material;
mod renderer;
mod shader_bundle;
mod shader_set;
mod skin_mesh;
mod texture_material;
mod ui_draw_data;
mod ui_mesh;
mod ui_scissor;
mod ui_texture_id;
mod ui_texture_material;
mod ui_vertex;
mod validation_counter;
mod vertex;
mod view_input;
#[cfg(test)]
mod view_input_tests;
mod vulkan;
mod vulkan_failure;

pub use clear_color::{クリアカラー, クリアカラーエラー};
pub use cloth_material::{布定数, 布素材, 布素材エラー};
pub use cloth_shader_set::布シェーダー一式;
pub use compute_shader::コンピュートシェーダー;
pub use draw_result::{描画結果, 見送り理由};
pub use error::レンダラーエラー;
pub use extent::ウィンドウ寸法;
pub use frame_input::{フレーム描画入力, 布フレーム入力};
pub use gpu_memory_stats::{GPUメモリ用途, GPUメモリ用途別確保量, GPUメモリ統計};
pub use material::マテリアル素材;
pub use particle_material::{粒子素材, 粒子素材エラー};
pub use particle_shader_set::{粒子シェーダー一式, 粒子シェーダー一式エラー};
pub use pbr_material::{マテリアル素材エラー, 金属粗さPBR素材};
pub use readback_image::読み戻し画像;
pub use readback_result::読み戻し結果;
pub use render_object_material::描画対象素材;
pub use render_scene_material::描画シーン素材;
pub use renderer::{CPU区間時間, レンダラー};
pub use shader_bundle::シェーダー束;
pub use shader_set::{シェーダー一式, シェーダー一式エラー};
pub use skin_mesh::{スキンメッシュ素材, スキンメッシュ素材エラー, スキン頂点属性};
pub use texture_material::{テクスチャ用途, テクスチャ素材, テクスチャ素材エラー};
pub use ui_draw_data::UI描画データ;
pub use ui_mesh::UIメッシュ;
pub use ui_scissor::UIシザー矩形px;
pub use ui_texture_id::UIテクスチャID;
pub use ui_texture_material::{UIテクスチャ素材, UIテクスチャ素材エラー};
pub use ui_vertex::UI頂点;
pub use validation_counter::検証カウンタ;
pub use vertex::頂点;
pub use view_input::{描画視点, 描画視点一覧};
pub use vulkan_failure::Vulkan失敗コード;
