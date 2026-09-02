//! レンダーグラフ中核: 仮想リソース・用途・パス宣言・バリア導出・実行器。
//! 波1でシーン描画・読み戻しコピー・提示遷移をこの上へ移行する
//! （外部から見える公開APIは変わらない。ここはblitz_render内部の機構）。
//! 波2でバッファ対応(コンピュートパス・GPU粒子トイ)を追加する。
//! 参照: `_doc/設計/レンダーグラフ.md`。

mod aspect;
mod barrier_apply;
mod barrier_derivation;
mod buffer_barrier_derivation;
mod buffer_registry;
mod buffer_state;
mod builder;
mod clear_spec;
mod color_attachments;
mod context;
mod depth_attachment;
mod executor;
mod handle;
mod initial_state;
mod pass;
mod pass_resource_usage;
mod registry;
mod rendering_setup;
mod state;
mod timestamp_write;
mod usage;

pub(crate) use aspect::画像アスペクト;
pub(crate) use builder::グラフ;
pub(crate) use clear_spec::クリア指定;
pub(crate) use color_attachments::カラー添付列;
pub(crate) use context::GPU命令の積み先と宣言済み資源の取り出し口;
pub(crate) use depth_attachment::深度アタッチメント;
pub(crate) use executor::グラフ実行器;
pub(crate) use handle::{バッファハンドル, 画像ハンドル};
pub(crate) use initial_state::{
    前フレームhdr読み直後状態, 前フレームコンピュート読み直後状態, 前フレームシャドウマップ読み直後状態, 前フレーム今のフレームの色読み直後状態,
    前フレーム動きベクトル書き込み直後状態, 前フレーム深度書き込み直後状態, 前フレーム画素段読み直後状態, 前フレーム粒子読み直後状態,
    前フレーム頂点入力読み直後状態, 取得直後の色画像状態, 局所可視度の画像の前フレーム直後状態, 履歴画像の前フレーム直後状態,
    焼いた画像を参照するだけのときの初期状態, 焼いた画像を焼き直すときの初期状態, 直前の送信でコンピュート書き直後状態,
};
pub(crate) use pass::{パス宣言, パス種別};
pub(crate) use state::画像状態;
pub(crate) use usage::{バッファ用途, 画像用途};
