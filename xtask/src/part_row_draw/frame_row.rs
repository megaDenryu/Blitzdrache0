//! 部品で組んだ一間四方の骨格の並びの検収入口。10棟と100棟の対を`scale_pair`の器へ渡すだけであり、
//! この階層が持つのは世界と出力ルートとシーン名と件数の対応だけである。
//!
//! **家の並びと世界を分けるのは、組み立ての形が違うものを別々の計器で見るためである。** 家は一体の階を積み、
//! 骨格は枠へ壁をはめる。同じ世界へ混ぜると、発行数が件数に依らないことが破れたときに、どちらの組み立てが
//! 壊したのかを絵と数から切り分けられない。
//!
//! シーン名を`prop_part_frame_row`にすることが、ベイ1つを近くで写す初期カメラと、書き換えもピクセル判定も持たない
//! 検収計画を選ばせる。10棟と100棟は同じシーン名を持ち、読む出力ルートだけが違う。
//! 参照: `_doc/設計/部品カタログと接合点.md`「骨格と壁の対、扉の対」

use std::process::ExitCode;

use super::bake_root::並びの実行時形式の置き場;
use super::baked_asset_file::焼き上がりを確かめるファイル;
use super::row_target::検収する並び1つ;
use super::scale_pair::件数を変えた並びの対;
use crate::acceptance::{検収の実行名, 検収シーン名};
use crate::asset_generator::世界名;

const 十棟: 検収する並び1つ = 検収する並び1つ::生成する(
    "10棟の一間四方の骨格の並び",
    世界名::一間四方の骨格を十棟建てた世界,
    並びの実行時形式の置き場::定数から生成する("target/part_frame_row_ten_assets"),
    焼き上がりを確かめるファイル::定数から生成する("target/part_frame_row_ten_assets/prop_part_frame_row.blitzasset"),
    検収の実行名::定数から生成する("ten"),
    10,
);

const 百棟: 検収する並び1つ = 検収する並び1つ::生成する(
    "100棟の一間四方の骨格の並び",
    世界名::一間四方の骨格を百棟建てた世界,
    並びの実行時形式の置き場::定数から生成する("target/part_frame_row_hundred_assets"),
    焼き上がりを確かめるファイル::定数から生成する("target/part_frame_row_hundred_assets/prop_part_frame_row.blitzasset"),
    検収の実行名::定数から生成する("hundred"),
    100,
);

const 一間四方の骨格の並びの対: 件数を変えた並びの対 = 件数を変えた並びの対::生成する(
    "part-frame-row-draw",
    "10棟と100棟の突き合わせ",
    検収シーン名::生成する("prop_part_frame_row"),
    "target/part_frame_row_draw",
    十棟,
    百棟,
);

pub(super) fn 実行する() -> ExitCode {
    一間四方の骨格の並びの対.実行する()
}
