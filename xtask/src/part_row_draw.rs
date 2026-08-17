//! 部品で組んだ並びを焼いて実機で描く検収の入口3つ。家の並びと一間四方の骨格の並びは件数を変えた対の実証を、
//! 木の並びは組み上がった木の姿の目視を担当する。
//!
//! 3つを1つのモジュールへ収めるのは、焼く工程が同じ1本の報告の行を出し、その読み取りを共有するためである
//! (`tally_line`)。読み取りを別々に持つと、綴りの写しが3つに分かれ、1つだけ直した食い違いが起きる。
//!
//! 家の並びと一間四方の骨格の並びはさらに検収の器そのものを共有する(`scale_pair`)。どちらも件数だけが違う2つの世界を
//! 焼いて突き合わせる形であり、違うのは世界と出力ルートとシーン名と件数だけである。その4つは`row_target`が持つ。
//!
//! 木の並びだけ器を分けるのは、見るものが違うためである。あちらは1本の姿を人が判定できる大きさで写すことが目的であり、
//! 件数を変えた対を持たない。

mod bake_root;
mod baked_asset_file;
mod frame_row;
mod house_row;
mod row_path_rule;
mod row_target;
mod scale_pair;
mod tally_line;
mod tree_row;

use std::process::ExitCode;

pub fn 家の並びを検収する() -> ExitCode {
    house_row::実行する()
}

pub fn 一間四方の骨格の並びを検収する() -> ExitCode {
    frame_row::実行する()
}

pub fn 木の並びを撮影する() -> ExitCode {
    tree_row::実行する()
}
