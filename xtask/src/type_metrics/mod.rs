//! 型単位の負債計測。conformのファイル単位検査(100行)では見えない、1つの型への責務集中を
//! 可視化する。implを複数ファイルへ散らせば行数検査は通過するが、Rustの可視性はモジュール
//! 単位で決まるため、同一モジュール木のすべてのファイルがその型の全privateフィールドへ触れる。
//! 参照: CLAUDE.md「切り出しの根拠義務（パーシャル規約、2026-07-25制定）」
//!
//! ここは計測と可視化だけを行い、閾値の判定は持たない。現状の値を上限として台帳へ登録し増えたら落とす
//! 検査は`conform/type_metrics_ledger`が持つ。計測の道具と、その値を規約として縛る台帳は別々に育つためである。
//!
//! 走査は構文解析器を持たない行単位の照合であり、次の精度限界がある。文字列リテラルとコメントの
//! 中にある struct・impl・fn・波括弧を実コードと区別できないため、誤検出と深さのずれが起こりうる。
//! マクロが生成する型とメソッドは行に現れないため数えられない。関数やimplの内側で入れ子に定義した
//! 型は数えない。型名はモジュールを跨いで素の名前で集計するため、同名の別型は合算される。
//! 列挙の枝は本体の直下で識別子から始まる行として数えるため、枝のタプルを複数行へ折り返した中身の行も
//! 1つの枝として数えてしまう。

mod body_kind;
mod declaration_amount;
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

pub use declaration_amount::宣言の分量;
pub use error::型計測の破れ;
pub use keyword::修飾子を取り除く;
/// 台帳の照合の試験が`型計測`を組み立てるために要る。実装は`型計測`のフィールドを通してしか触らないため、
/// 試験のときだけ名前を出す。
#[cfg(test)]
pub use metrics::型の宣言;
pub use metrics::{型計測, 集計する};
pub use observation::観測;

const 走査対象ディレクトリ一覧: [&str; 2] = ["crates", "xtask/src"];
const 表示件数: usize = 20;

pub fn 型ごとの分量を計測する() -> ExitCode {
    match ファイル別の観測を集める() {
        Ok(ファイル別観測) => {
            report::上位を表示する(&集計する(&ファイル別観測), 表示件数);
            ExitCode::SUCCESS
        }
        Err(誤り) => {
            eprintln!("[xtask] type-metricsを実行できなかった: {誤り}");
            ExitCode::FAILURE
        }
    }
}

/// 走査対象のRustファイルを1本ずつ読み、ファイルごとの観測へ写す。conformの台帳検査と自由関数の検査が
/// 同じ走査を使うため、コマンドの表示から切り離してここを共通の入口にしている。
pub fn ファイル別の観測を集める() -> Result<Vec<(PathBuf, Vec<観測>)>, 型計測の破れ> {
    let ファイル一覧 = file_scan::対象ファイル一覧を集める(&走査対象ディレクトリ一覧, &["rs"])?;
    let mut 結果 = Vec::new();
    for パス in ファイル一覧 {
        let 内容 = std::fs::read_to_string(&パス)
            .map_err(|誤り| 型計測の破れ::計測対象のファイルを読めなかった {
                パス: パス.clone(), 誤り
            })?;
        結果.push((パス, scan::走査する(&内容)));
    }
    Ok(結果)
}
