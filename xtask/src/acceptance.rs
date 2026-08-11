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
//! 誤りの型は`error`、読み戻した絵は`readback_image`、その置き場は`readback_dump`、
//! アプリが読む実行時形式の置き場は`runtime_asset_root`、アプリの終了時報告は`exit_report`、
//! 起こし方の綴りは`app_executable`、1回ぶんの起動の指定は`launch_specification`、
//! 画像の寸法と画素の位置は`pixel_geometry`が、1回の実行を指す名前は`run_name`が、
//! それらを束ねる操作サービスは`run_environment`が持つ。

mod app_executable;
mod error;
mod exit_report;
mod launch_specification;
mod pixel_geometry;
mod readback_dump;
mod readback_image;
mod run_environment;
mod run_name;
mod runtime_asset_root;

pub use app_executable::アプリの起こし方;
pub use error::検収エラー;
pub use exit_report::終了時報告;
pub use launch_specification::{アプリの起動指定, 描画フレーム数, 検収シーン名};
pub use pixel_geometry::{画像の幅, 画像の高さ, 画素の横位置, 画素の番号, 画素の縦位置};
pub use readback_dump::{読み戻しの書き出し先, 読み戻しの置き場};
pub use readback_image::読み戻し画像;
pub use run_environment::{描画検収の実行環境, 検収の1回の実行};
pub use run_name::検収の実行名;
pub use runtime_asset_root::実行時アセットルート;
