//! レンダーグラフ + リソースシステム。エンジン中核の型安全層。
//!
//! 責務: Vulkan(ash) の unsafe をこのクレートの実装内部に封じ込め、
//! 上位層には「このエンジンの使い方」だけを表現した安全な型を公開する。
//!
//! 注意: 公開APIに ash の型（vk::* やハンドル）を露出してはならない。
//! 露出した瞬間、unsafe の封じ込めが破れて上位層に責任が漏れる。

mod clear_color;
mod draw_result;
mod error;
mod extent;
mod renderer;
mod validation_counter;
mod vulkan;
mod vulkan_failure;

pub use clear_color::{クリアカラー, クリアカラーエラー};
pub use draw_result::{描画結果, 見送り理由};
pub use error::レンダラーエラー;
pub use extent::ウィンドウ寸法;
pub use renderer::レンダラー;
pub use validation_counter::検証カウンタ;
pub use vulkan_failure::Vulkan失敗コード;
