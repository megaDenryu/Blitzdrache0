//! レンダーグラフ中核: 仮想リソース・用途・パス宣言・バリア導出・実行器。
//! 波1でシーン描画・読み戻しコピー・提示遷移をこの上へ移行する
//! （外部から見える公開APIは変わらない。ここはblitz_render内部の機構）。
//! 参照: `_doc/設計/レンダーグラフ.md`。

mod aspect;
mod barrier_apply;
mod barrier_derivation;
mod builder;
mod clear_spec;
mod context;
mod executor;
mod handle;
mod initial_state;
mod pass;
mod pass_resource_usage;
mod registry;
mod rendering_setup;
mod state;
mod usage;

pub(crate) use aspect::画像アスペクト;
pub(crate) use builder::グラフ;
pub(crate) use clear_spec::クリア指定;
pub(crate) use executor::実行する;
pub(crate) use handle::画像ハンドル;
pub(crate) use initial_state::{前フレーム深度書き込み直後状態, 取得直後の色画像状態};
pub(crate) use pass::{パス宣言, パス種別};
pub(crate) use usage::画像用途;
