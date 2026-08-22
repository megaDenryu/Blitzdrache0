//! 部品で組んだ家の並びの検収入口。10軒と100軒の対を`scale_pair`の器へ渡すだけであり、この階層が持つのは
//! 世界と出力ルートとシーン名と件数の対応だけである。
//!
//! **これが段4の実証である。** 発行数が件数に比例し始めたら、部品の組み合わせでバリエーションを作るという
//! 設計は前提から誤っている。
//!
//! シーン名を`prop_part_house_row`にすることが、家並みの全体を収める初期カメラと、書き換えもピクセル判定も持たない
//! 検収計画を選ばせる。10軒と100軒は同じシーン名を持ち、読む出力ルートだけが違う。
//! 参照: `_doc/設計/部品カタログと接合点.md`「検収の形」段4の完了条件

use std::process::ExitCode;

use super::bake_root::並びの実行時形式の置き場;
use super::baked_asset_file::焼き上がりを確かめるファイル;
use super::row_target::検収する並び1つ;
use super::scale_pair::件数を変えた並びの対;
use crate::acceptance::{検収の実行名, 検収シーン名};
use crate::asset_generator::世界名;

const 十軒: 検収する並び1つ = 検収する並び1つ::生成する(
    "10軒の家並み",
    世界名::部品で建てた十軒の世界,
    並びの実行時形式の置き場::定数から生成する("target/part_house_row_ten_assets"),
    焼き上がりを確かめるファイル::定数から生成する("target/part_house_row_ten_assets/prop_part_house_row.blitzasset"),
    検収の実行名::定数から生成する("ten"),
    10,
);

const 百軒: 検収する並び1つ = 検収する並び1つ::生成する(
    "100軒の家並み",
    世界名::部品で建てた百軒の世界,
    並びの実行時形式の置き場::定数から生成する("target/part_house_row_hundred_assets"),
    焼き上がりを確かめるファイル::定数から生成する("target/part_house_row_hundred_assets/prop_part_house_row.blitzasset"),
    検収の実行名::定数から生成する("hundred"),
    100,
);

const 家の並びの対: 件数を変えた並びの対 = 件数を変えた並びの対::生成する(
    "part-house-row-draw",
    "10軒と100軒の突き合わせ",
    検収シーン名::生成する("prop_part_house_row"),
    "target/part_house_row_draw",
    十軒,
    百軒,
);

pub(super) fn 家の並びの対を検収する() -> ExitCode {
    家の並びの対.検収してコードへ変換する()
}
