//! 検収の共通語彙: 61のコマンドが「blitz_appを起こして絵と報告を読む」ときに共通で使う型の集まり。
//!
//! 層と依存の向きを先に決める。ここはコマンドを1つも知らず、コマンドの側だけがここを知る。
//! この向きを守ると、器を直したときに直る範囲がコマンド全部になり、コマンドを足しても器は動かない。
//!
//! 器を1箇所に置くのは、同じ3手(起動する・検証層の指摘が零であることを確かめる・読み戻す)を
//! コマンドごとに書き直していたためである。実測では、寸法とRGBA8を持つ構造体が11のファイルで別々に定義され、
//! 画素を読むメソッドが10回書き直されていた。同じ手順の写しが増えると、読み戻しの書式が変わったときに
//! 直し漏れた入口だけが静かに壊れる。
//!
//! 誤りの型は`error`、読み戻した絵は`readback_image`、明るさの圧縮を通す前の絵は`readback_hdr_image`、
//! その置き場は`readback_dump`、
//! 書き出された寸法の読み取りは`readback_size`、報告の1行と区画は`report_line`と`report_section`、
//! アプリが読む実行時形式の置き場は`runtime_asset_root`、アプリの終了時報告は`exit_report`、
//! 起こし方の綴りは`app_executable`、1回ぶんの起動の組み立てと実行は`app_launch`、
//! 起動へ渡すシーンは`scene_name`、枚数は`frame_count`、それらと選択肢を束ねた指定は`launch_specification`、
//! 判定を名指す名前とその名前が課す比べは`judgment_name`が、判定が比べる値は`judgment_value`が、
//! 画像の寸法と画素の位置は`pixel_geometry`が、画素の数どうしの比は`pixel_ratio`が、1回の実行を指す名前は`run_name`が、
//! 世界を読ませて報告だけを採る操作サービスは`world_report_environment`が、
//! 世界を読まずに報告だけを採る操作サービスは`report_only_environment`が、
//! 絵を読む操作サービスは`run_environment`が持つ。

mod app_executable;
mod app_launch;
mod draw_end;
mod dump_format;
mod error;
mod exit_report;
mod frame_count;
mod judgment_name;
mod judgment_value;
mod launch_arguments;
mod launch_specification;
mod pixel_geometry;
mod pixel_ratio;
mod readback_dump;
mod readback_hdr_image;
mod readback_image;
mod readback_size;
mod report_line;
mod report_only_environment;
mod report_section;
mod reserved_option;
mod run_environment;
mod run_name;
mod runtime_asset_root;
mod scene_name;
#[cfg(test)]
mod test_output_place;
mod world_report_environment;

pub use app_executable::アプリの起こし方;
pub use dump_format::書き出しの形式;
pub use error::{判定の破れ, 判定の破れの種類, 検収エラー};
pub use exit_report::終了時報告;
pub use frame_count::描画フレーム数;
pub use judgment_name::判定の名前;
pub use judgment_value::判定が比べる値;
pub use launch_specification::アプリの起動指定;
pub use pixel_geometry::{画像の幅, 画像の高さ, 画素の横位置, 画素の番号, 画素の縦位置};
pub use pixel_ratio::画素の占める割合;
pub use readback_hdr_image::圧縮前のHDR画像;
pub use readback_image::読み戻し画像;
pub use report_line::報告の行;
pub use report_only_environment::世界を読まずに報告を採る実行環境;
pub use report_section::報告の区画;
pub use run_environment::{描画検収の実行環境, 検収の1回の実行};
pub use run_name::検収の実行名;
pub use runtime_asset_root::実行時アセットルート;
pub use scene_name::検収シーン名;
pub use world_report_environment::世界を読ませて報告を採る実行環境;
