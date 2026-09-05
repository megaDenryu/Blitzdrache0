//! 検証の標準列。参照: CLAUDE.md「機械的強制」
//!
//! 各段の標準出力と標準エラーは、端末とログのファイルの両方へ同じ流れが行く。ログを書くのは道具自身であり、
//! 呼び出し側のシェルの転送ではない。読む側(実装員と親)がエージェントの要約でなくログの実物を読むためである。
//! 参照: issue #82

mod clean_build_cache;
mod log_error;
mod log_file_name;
mod log_place;
mod output;
mod sequence;
mod tee;
mod utc_moment;

use std::process::ExitCode;

pub(crate) use clean_build_cache::{ビルドの中間データを掃除する, 消さずに一覧だけ出す旗};
pub(crate) use output::{検証の出力のファイル名, 検証の出力の置き場名, 検証の出力ルート};

use log_error::検証列の破れ;
use log_file_name::検証のログのファイル名;
use log_place::検証のログの置き場;
use sequence::検証列の実行係;

pub fn 検証列を実行する() -> ExitCode {
    match ログを開いて検証列を走らせる() {
        Ok(終了コード) => 終了コード,
        Err(破れ) => {
            eprintln!("[xtask] 検証列を走らせられなかった: {破れ}");
            ExitCode::FAILURE
        }
    }
}

fn ログを開いて検証列を走らせる() -> Result<ExitCode, 検証列の破れ> {
    let 置き場 = 検証のログの置き場::いま使っている木から決める();
    let ファイル名 = 検証のログのファイル名::ブランチと先端と時刻から組み立てる()?;
    let ログのパス = 置き場.ログのファイルの絶対パスを用意する(&ファイル名)?;
    検証列の実行係::ログを開いて作る(ログのパス)?.ログの場所を告げて全段を走らせる()
}
