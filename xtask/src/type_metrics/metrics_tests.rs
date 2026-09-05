//! ファイル別の観測から型ごとの計測へ畳む集計の試験。複数ファイルのimplを合算して降順に並べることと、
//! 同じ名前で場所の違う型を別々に数えることと、同じファイルに同名の定義が2つあるとき大きい側が
//! 小さい側に隠れないことの3つを固定する。
#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use super::declaration_amount::宣言の分量;
use super::file_observation::ファイルの観測;
use super::import_index::取り込みの索引;
use super::metrics::集計する;
use super::observation::観測;
use super::type_path::自己型の経路;

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

fn ファイル(パス: &str, 観測一覧: Vec<観測>) -> ファイルの観測 {
    ファイルの観測 {
        パス: PathBuf::from(パス),
        観測一覧,
        取り込みの索引: 取り込みの索引::ファイルの内容から生成する(""),
    }
}

#[test]
fn 複数ファイルのimplを合算して降順に並べる() {
    let 観測 = vec![
        ファイル("src/a.rs", vec![定義("大", 宣言の分量::構造体のフィールド数(3)), 実装("大", 2)]),
        ファイル("src/b.rs", vec![実装("大", 1), 定義("小", 宣言の分量::列挙の枝数(1))]),
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
        ),
        ファイル(
            "xtask/src/smoke/launch_setting.rs",
            vec![定義("起動設定", 宣言の分量::構造体のフィールド数(10)), 実装("起動設定", 9)],
        ),
    ];
    let 一覧 = 集計する(&観測).型ごとの計測一覧;
    assert_eq!(一覧.len(), 2);
    let 綴り一覧: Vec<String> = 一覧.iter().map(|計測| format!("{}:{}", 計測.所在, 計測.宣言の件数())).collect();
    assert!(綴り一覧.contains(&"crates/blitz_app/src/cli/types.rs::起動設定:40".to_string()));
    assert!(綴り一覧.contains(&"xtask/src/smoke/launch_setting.rs::起動設定:10".to_string()));
}

/// `#[cfg]`で切り替わる同名の定義が同じファイルに並ぶ形である。後から現れた定義で上書きすると、
/// 40フィールドの型が3フィールドの型に隠れて閾値の検査をすり抜ける(Issue #68と同じ隠れ方)。
#[test]
fn 同じファイルの同名の定義は小さい側で大きい側を隠さない() {
    let 観測 = vec![ファイル(
        "src/a.rs",
        vec![
            定義("設定", 宣言の分量::構造体のフィールド数(40)),
            定義("設定", 宣言の分量::構造体のフィールド数(3)),
        ],
    )];
    let 一覧 = 集計する(&観測).型ごとの計測一覧;
    assert_eq!(一覧.len(), 1);
    assert_eq!(一覧[0].宣言の件数(), 40);
}
