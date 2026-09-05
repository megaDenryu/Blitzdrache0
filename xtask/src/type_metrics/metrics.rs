//! 型ごとの計測値と、ファイル別の観測からの集計。
//! 型は定義ファイルと型名の組(`型の所在`)で識別するため、同じ名前の型が別の場所にあれば別々に数える。

use std::collections::BTreeSet;
use std::path::PathBuf;

use super::attribution_input::引き当ての材料;
use super::declaration_amount::宣言の分量;
use super::definition_index::定義の索引;
use super::file_observation::ファイルの観測;
use super::impl_attribution::定義の候補が複数ある実装ブロック;
use super::measurement_table::所在ごとの計測表;
use super::observation::観測;
use super::type_location::型の所在;

pub struct 型計測 {
    pub 所在: 型の所在,
    pub 宣言: Option<宣言の分量>,
    pub 実装ファイル一覧: BTreeSet<PathBuf>,
    pub メソッド総数: usize,
}

impl 型計測 {
    pub fn 所在だけで始める(所在: 型の所在) -> Self {
        Self {
            所在,
            宣言: None,
            実装ファイル一覧: BTreeSet::new(),
            メソッド総数: 0,
        }
    }

    /// 宣言が持つ数。構造体ならフィールド数、列挙なら枝の数であり、宣言が走査に現れない型は0とする。
    pub fn 宣言の件数(&self) -> usize {
        self.宣言.map_or(0, |分量| 分量.件数())
    }

    /// 降順に並べるための比較鍵。implの分散ファイル数を最優先し、次に宣言の件数、最後にメソッド総数で比べる。
    /// 分散ファイル数を先頭に置くのは、privateフィールドへ触れるファイルの数がそのまま責務の広がりを表すためである。
    pub fn 比較鍵(&self) -> (usize, usize, usize) {
        (self.実装ファイル一覧.len(), self.宣言の件数(), self.メソッド総数)
    }
}

/// 走査範囲の全ファイルから得た計測の一式。引き当てられなかったimplブロックを一緒に返すのは、
/// 解析できなかった入力を黙って対象から外さないためである。
pub struct 走査範囲の型計測 {
    pub 型ごとの計測一覧: Vec<型計測>,
    pub 定義の候補を1つに絞れなかった実装ブロック一覧: Vec<定義の候補が複数ある実装ブロック>,
}

pub fn 集計する(ファイル別観測: &[ファイルの観測]) -> 走査範囲の型計測 {
    let 索引 = 定義の索引::ファイル別の観測から生成する(ファイル別観測);
    let mut 計測表 = 所在ごとの計測表::空で始める();
    for ファイル in ファイル別観測 {
        for 観測 in &ファイル.観測一覧 {
            match 観測 {
                観測::型定義 { 型名, 分量 } => {
                    計測表.型定義を取り込む(型の所在::走査したファイルから生成する(&ファイル.パス, 型名), *分量);
                }
                観測::実装ブロック {
                    自己型の経路, メソッド数
                } => {
                    let 材料 = 引き当ての材料::生成する(&ファイル.パス, 自己型の経路, &ファイル.取り込みの索引);
                    計測表.実装ブロックを取り込む(索引.実装ブロックの所在を引き当てる(&材料), &ファイル.パス, *メソッド数);
                }
            }
        }
    }
    計測表.分量の降順へ並べて閉じる()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::type_metrics::import_index::取り込みの索引;
    use crate::type_metrics::type_path::自己型の経路;

    fn 定義(型名: &str, 分量: 宣言の分量) -> 観測 {
        let 型名 = 型名.to_string();
        観測::型定義 { 型名, 分量 }
    }

    fn 実装(経路の綴り: &str, メソッド数: usize) -> 観測 {
        観測::実装ブロック {
            自己型の経路: 自己型の経路::綴りから生成する(経路の綴り),
            メソッド数,
        }
    }

    fn ファイル(パス: &str, 観測一覧: Vec<観測>, 取り込み: &str) -> ファイルの観測 {
        ファイルの観測 {
            パス: PathBuf::from(パス),
            観測一覧,
            取り込みの索引: 取り込みの索引::ファイルの内容から生成する(取り込み),
        }
    }

    #[test]
    fn 複数ファイルのimplを合算して降順に並べる() {
        let 観測 = vec![
            ファイル("src/a.rs", vec![定義("大", 宣言の分量::構造体のフィールド数(3)), 実装("大", 2)], ""),
            ファイル("src/b.rs", vec![実装("大", 1), 定義("小", 宣言の分量::列挙の枝数(1))], ""),
        ];
        let 一覧 = 集計する(&観測).型ごとの計測一覧;
        assert_eq!(一覧[0].所在.to_string(), "src/a.rs::大");
        assert_eq!(一覧[0].実装ファイル一覧.len(), 2);
        assert_eq!(一覧[0].メソッド総数, 3);
        assert_eq!(一覧[1].所在.to_string(), "src/b.rs::小");
        assert_eq!(一覧[1].宣言.unwrap().指標名(), "枝数");
    }

    #[test]
    fn 同じ名前の型が2つあれば別々の型として数える() {
        let 観測 = vec![
            ファイル(
                "crates/blitz_app/src/cli/types.rs",
                vec![定義("起動設定", 宣言の分量::構造体のフィールド数(40))],
                "",
            ),
            ファイル(
                "xtask/src/smoke/launch_setting.rs",
                vec![定義("起動設定", 宣言の分量::構造体のフィールド数(10)), 実装("起動設定", 9)],
                "",
            ),
        ];
        let 一覧 = 集計する(&観測).型ごとの計測一覧;
        assert_eq!(一覧.len(), 2);
        let 綴り一覧: Vec<String> = 一覧.iter().map(|計測| format!("{}:{}", 計測.所在, 計測.宣言の件数())).collect();
        assert!(綴り一覧.contains(&"crates/blitz_app/src/cli/types.rs::起動設定:40".to_string()));
        assert!(綴り一覧.contains(&"xtask/src/smoke/launch_setting.rs::起動設定:10".to_string()));
    }
}
