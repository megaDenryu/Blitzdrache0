//! 台帳の中身を引く試験。照合の規則でなく、いま登録されている行そのものを見る。
//!
//! 台帳は現状の写しであり書き換わるため、見るのは書き換えても壊れない2つだけである。
//! 1つは台帳に載っている型を所在で引けること、もう1つは同じ名前で場所の違う2つの型を別の行として引くことである。

use std::path::PathBuf;

use crate::type_metrics::{型の所在, 宣言の分量};

use super::tests::計測;
use super::{ledger, 計測1件を照合する};

#[test]
fn 台帳に載っているのに計測へ現れない型を違反にする() {
    let 台帳 = ledger::全項目();
    assert!(!ledger::計測に現れない項目一覧(&台帳, &[]).is_empty());
    let アプリの所在 = 型の所在::台帳の綴りから生成する("crates/blitz_app/src/app/mod.rs", "アプリ");
    assert!(ledger::上限を参照する(&台帳, &アプリの所在).is_some());
}

/// 起動設定という名前の型は2つある。名前だけを鍵にしていた頃は、40欄の側が10欄の側に隠れて検査に現れず、
/// 台帳へ載せても違反の報告が10欄の側のファイルを指した。この試験がその再発を止める。
#[test]
fn 同じ名前でも定義ファイルが違えば別の行として台帳を引く() {
    let 台帳 = ledger::全項目();
    let 起動設定 = |定義ファイル: &str, フィールド数: usize, メソッド総数: usize| {
        計測(定義ファイル, "起動設定", 1, 宣言の分量::構造体のフィールド数(フィールド数), メソッド総数)
    };
    assert!(計測1件を照合する(&台帳, &起動設定("crates/blitz_app/src/cli/types.rs", 40, 1)).is_empty());
    assert!(計測1件を照合する(&台帳, &起動設定("xtask/src/smoke/launch_setting.rs", 10, 8)).is_empty());
    let 違反一覧 = 計測1件を照合する(&台帳, &起動設定("crates/blitz_app/src/cli/types.rs", 41, 1));
    assert_eq!(違反一覧.len(), 1);
    assert_eq!(違反一覧[0].パス, PathBuf::from("crates/blitz_app/src/cli/types.rs"));
    assert!(違反一覧[0].説明.contains("フィールド数が40から41へ増えた"));
}
