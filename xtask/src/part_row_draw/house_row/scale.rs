//! 家の並びの規模: 何軒建てた世界を焼いて撮るかを指す枝。世界名と出力ルートと実行時形式の置き場と実行名を、
//! この1つの枝が同時に決める。
//!
//! 4つを1つの枝から引くのは、規模ごとに別々の綴りを呼び出し側が組むと、10軒の世界を焼いて100軒の実行時形式を
//! 読む組み合わせが型を通るためである。規模を選べば行き先がすべて決まる。
//!
//! 前回の焼き上がりを消す口をこの型が持つのは、消してよい置き場を知っているのがこの型だけだからである。

use std::path::Path;

use crate::acceptance::{実行時アセットルート, 検収の実行名};
use crate::asset_generator::世界名;
use crate::compile_assets::{部品で建てた十軒の出力ルート, 部品で建てた百軒の出力ルート};
use crate::world_setup::検収世界の用意;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum 家の並びの規模 {
    十軒,
    百軒,
}

impl 家の並びの規模 {
    /// 焼いて撮る規模を、報告に出したい順で並べる。少ないほうを先に置くのは、失敗したときに
    /// 焼く時間の短い側で先に落ちるためである。
    pub(super) fn 全規模() -> [Self; 2] {
        [Self::十軒, Self::百軒]
    }

    pub(super) fn 呼び名(self) -> &'static str {
        match self {
            Self::十軒 => "10軒の家並み",
            Self::百軒 => "100軒の家並み",
        }
    }

    pub(super) fn 世界(self) -> 世界名 {
        match self {
            Self::十軒 => 世界名::部品で建てた十軒の世界,
            Self::百軒 => 世界名::部品で建てた百軒の世界,
        }
    }

    pub(super) fn 出力ルート(self) -> &'static Path {
        match self {
            Self::十軒 => 部品で建てた十軒の出力ルート(),
            Self::百軒 => 部品で建てた百軒の出力ルート(),
        }
    }

    pub(super) fn 実行時アセットルート(self) -> 実行時アセットルート {
        実行時アセットルート::綴りから生成する(match self {
            Self::十軒 => "target/part_house_row_ten_assets",
            Self::百軒 => "target/part_house_row_hundred_assets",
        })
    }

    /// 焼き上がりを名指しで確かめる相手。外部のアセットリポジトリが無い環境では部品の宣言が飛ばされ、
    /// 実行時形式が作られないまま焼く工程が成功する。
    pub(super) fn 焼き上がりの確かめ方(self) -> 検収世界の用意 {
        match self {
            Self::十軒 => 検収世界の用意::生成する("10軒の家並み", "target/part_house_row_ten_assets/prop_part_house_row.blitzasset"),
            Self::百軒 => {
                検収世界の用意::生成する("100軒の家並み", "target/part_house_row_hundred_assets/prop_part_house_row.blitzasset")
            }
        }
    }

    /// 焼く前に前回の焼き上がりを消す。**この入口は焼く工程の報告の行を期待値に使うため、
    /// 生成台帳が据え置きを選ぶと行が1本も出ず、比べる相手を失う。** 消してよいのは、この置き場を
    /// この入口だけが使うためである。まだ無いことは破れではない。
    pub(super) fn 前回の焼き上がりを消す(self) -> std::io::Result<()> {
        match std::fs::remove_dir_all(self.出力ルート()) {
            Err(誤り) if 誤り.kind() == std::io::ErrorKind::NotFound => Ok(()),
            他 => 他,
        }
    }

    /// 絵のファイル名になり、失敗の文面もこの名前で実行を名指す。
    pub(super) fn 実行名(self) -> 検収の実行名 {
        match self {
            Self::十軒 => 検収の実行名::定数から生成する("ten"),
            Self::百軒 => 検収の実行名::定数から生成する("hundred"),
        }
    }
}
