//! 型単位の負債計測。conformのファイル単位検査(100行)では見えない、1つの型への責務集中を
//! 可視化する。implを複数ファイルへ散らせば行数検査は通過するが、Rustの可視性はモジュール
//! 単位で決まるため、同一モジュール木のすべてのファイルがその型の全privateフィールドへ触れる。
//! 参照: CLAUDE.md「切り出しの根拠義務（パーシャル規約、2026-07-25制定）」
//!
//! 閾値による違反判定はここでは行わず、計測と可視化だけを行う。現状で最大の型が確実に閾値を
//! 超えるため、denyにすると検証列全体が赤くなり他のすべての作業が止まる。閾値でのdenyは型の
//! 分解が完了した後に別途入れる。
//!
//! 走査は構文解析器を持たない行単位の照合であり、次の精度限界がある。文字列リテラルとコメントの
//! 中にある struct・impl・fn・波括弧を実コードと区別できないため、誤検出と深さのずれが起こりうる。
//! マクロが生成する型とメソッドは行に現れないため数えられない。関数やimplの内側で入れ子に定義した
//! 型は数えない。型名はモジュールを跨いで素の名前で集計するため、同名の別型は合算される。

mod body_kind;
mod definition_line;
mod error;
mod impl_line;
mod keyword;
mod member_line;
mod metrics;
mod observation;
mod report;
mod scan;

use std::path::PathBuf;
use std::process::ExitCode;

use crate::file_scan;
use error::型計測の破れ;
use observation::観測;

const 走査対象ディレクトリ一覧: [&str; 2] = ["crates", "xtask/src"];
const 表示件数: usize = 20;

pub fn 実行する() -> ExitCode {
    let ファイル一覧 = match file_scan::対象ファイル一覧を集める(&走査対象ディレクトリ一覧, &["rs"]) {
        Ok(一覧) => 一覧,
        Err(誤り) => {
            eprintln!("[xtask] type-metricsのファイル走査に失敗した: {誤り}");
            return ExitCode::FAILURE;
        }
    };
    let ファイル別観測 = match 全ファイルを走査する(&ファイル一覧) {
        Ok(観測) => 観測,
        Err(誤り) => {
            eprintln!("[xtask] type-metricsのファイル読み取りに失敗した: {誤り}");
            return ExitCode::FAILURE;
        }
    };
    report::上位を表示する(&metrics::集計する(&ファイル別観測), 表示件数);
    ExitCode::SUCCESS
}

fn 全ファイルを走査する(ファイル一覧: &[PathBuf]) -> Result<Vec<(PathBuf, Vec<観測>)>, 型計測の破れ> {
    let mut 結果 = Vec::new();
    for パス in ファイル一覧 {
        let 内容 = std::fs::read_to_string(パス)
            .map_err(|誤り| 型計測の破れ::計測対象のファイルを読めなかった {
                パス: パス.clone(), 誤り
            })?;
        結果.push((パス.clone(), scan::走査する(&内容)));
    }
    Ok(結果)
}
