//! 台帳の1行と、1つの区画分の一覧。区画とは、同じモジュールの根を共有する行のまとまりのことである。
//!
//! 台帳の行がパスを根からの相対で持つのは、綴りを短くして1項目を1行に収めるためである。全体のパスを
//! 毎項目へ書くと整形が1項目を6行へ折り返し、1つの表が1ファイル100行の原則を超える。根は区画の側が1箇所で持つ。
//! 様式は`xtask/src/conform/free_function_whole_type/ledger/entry.rs`に倣う。

use crate::type_metrics::{型の所在, 宣言の分量};

use super::super::limit::型ごとの上限;

pub struct 台帳の行 {
    根からのパス: &'static str,
    型名: &'static str,
    実装ファイル数: usize,
    宣言の上限: 宣言の分量,
    メソッド総数: usize,
}

impl 台帳の行 {
    pub const fn 構造体(
        根からのパス: &'static str, 型名: &'static str, 実装ファイル数: usize, フィールド数: usize, メソッド総数: usize
    ) -> Self {
        Self {
            根からのパス,
            型名,
            実装ファイル数,
            宣言の上限: 宣言の分量::構造体のフィールド数(フィールド数),
            メソッド総数,
        }
    }

    pub const fn 列挙(
        根からのパス: &'static str, 型名: &'static str, 実装ファイル数: usize, 枝数: usize, メソッド総数: usize
    ) -> Self {
        Self {
            根からのパス,
            型名,
            実装ファイル数,
            宣言の上限: 宣言の分量::列挙の枝数(枝数),
            メソッド総数,
        }
    }
}

pub struct 区画の一覧 {
    モジュールの根: &'static str,
    行一覧: &'static [台帳の行],
    自分のファイル: &'static str, // 陳腐化の違反がこの表を指せるよう、表の側が`file!()`で名乗る
}

impl 区画の一覧 {
    pub const fn 生成する(モジュールの根: &'static str, 行一覧: &'static [台帳の行], 自分のファイル: &'static str) -> Self {
        Self {
            モジュールの根,
            行一覧,
            自分のファイル,
        }
    }

    pub fn 上限一覧(&self) -> Vec<型ごとの上限> {
        self.行一覧
            .iter()
            .map(|行| 型ごとの上限 {
                所在: 型の所在::台帳の綴りから生成する(&format!("{}/{}", self.モジュールの根, 行.根からのパス), 行.型名),
                実装ファイル数: 行.実装ファイル数,
                宣言の上限: 行.宣言の上限,
                メソッド総数: 行.メソッド総数,
                登録されている表のファイル: self.自分のファイル,
            })
            .collect()
    }
}
