//! blitz_render 全体で使う型付きエラー。層ごとの失敗の型と、それらを畳む`レンダラーエラー`の置き場所をまとめる。
//! 参照: CLAUDE.md「エラー・パニック」。Vulkanの実行時失敗は握り潰さずここへ集約し、呼び出し元へ `?` で伝播する。

mod cascade;
mod cloth;
mod conversions;
pub(crate) mod device_requirement;
mod frame_input_mismatch;
mod lighting_query;
mod material_table;
mod pipeline_ledger;
mod renderer_error;
mod sky;

pub use cascade::多段エラー;
pub use cloth::布エラー;
pub use device_requirement::{ディスクリプタ索引機能項目, デバイス要件エラー};
pub use frame_input_mismatch::フレーム入力不一致エラー;
pub use lighting_query::照明問い合わせ梱包エラー;
pub use material_table::材質資源表エラー;
pub use pipeline_ledger::パイプライン台帳エラー;
pub use renderer_error::レンダラーエラー;
pub use sky::空エラー;
