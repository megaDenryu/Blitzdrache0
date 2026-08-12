//! 1フレームの記録・送信・提示。dynamic rendering + synchronization2で行う。
//! 記録を頼む側がデバイスとキューとコマンドバッファを運ばずに済むよう、局面はセッション型のメソッドとして開ける
//! (積み始めは`environment`と`session`、積むのは`record`、閉じて送信し提示するのは`submit_present`)。

mod bind_tally;
mod cloth_types;
mod copy;
mod dispatch;
mod draw_switch_tally;
mod environment;
mod images;
mod point_light_shadow_tally;
mod point_light_shadow_types;
mod post_types;
mod present;
pub(crate) mod record;
mod record_counts;
mod record_metrics;
mod session;
mod shadow_types;
mod shared_set_bind;
mod sky_types;
mod submit_outcome;
mod submit_present;
mod types;
mod ui_types;

mod acquire;
pub(crate) mod depth_prepass_draw;
pub(crate) mod draw_commands;

pub(crate) use crate::vulkan::descriptor::共有セット束縛;
pub(crate) use acquire::取得結果;
pub(crate) use bind_tally::{セット別束縛計数, セット番号の数};
pub(crate) use cloth_types::{布シャドウ描画入力, 布描画の外部資源, 布描画入力};
pub(crate) use dispatch::{任意描画入力, 同期入力, 描画対象入力, 提示先};
pub(crate) use draw_switch_tally::記録側切替計数;
pub(crate) use environment::フレームの記録の実行環境;
pub(crate) use images::{フレーム画像一式, 光のにじみ画像};
pub(crate) use point_light_shadow_types::{点光源の影の描画発行, 点光源の影の束縛};
pub(crate) use post_types::{光のにじみ描画入力, 明るさの圧縮描画入力};
pub(crate) use record::{フレームの記録の材料, 記録の実績};
pub use record_counts::記録側の計数;
pub(crate) use record_metrics::記録の計器;
pub(crate) use session::フレームのGPU命令を積むコマンドバッファ;
pub(crate) use shadow_types::{シャドウ描画入力, 距離区分別のシャドウ入力};
pub(crate) use sky_types::{空中遠近合成描画入力, 空描画入力};
pub(crate) use submit_outcome::送信後の結末;
pub(crate) use types::{ジオメトリ入力, スキニング描画入力, 描画方式, 粒子描画入力};
pub(crate) use ui_types::{UI描画入力, UI描画項目};
