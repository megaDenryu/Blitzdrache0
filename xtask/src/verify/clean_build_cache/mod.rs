//! ビルドの中間データの掃除の入口。差分ビルドの中間データ・codexレビュー用のビルドの出力先・
//! 作業ツリーごとのビルドの出力先を、実在するものだけ消し、消した対象と解放した容量を1行ずつ表示する。
//!
//! Cargoは条件が変わって別の単位になった中間データを消さないため、掃除の仕組みがCargo自体に無い。
//! 開発者が消さない限り増え続け、3日の作業で500件を超えた実測がある。
//! 参照: [Issue #61](https://github.com/megaDenryu/Blitzdrache0/issues/61)。

mod candidate;
mod collect;
mod error;
mod mode;
mod occupied_size;
mod role;
mod subject;

use std::process::ExitCode;

use error::掃除の破れ;
use mode::実行モード;
use occupied_size::占める容量;

pub(crate) use mode::消さずに一覧だけ出す旗;

pub(crate) fn ビルドの中間データを掃除する(引数一覧: &[String]) -> ExitCode {
    let モード = match 実行モード::引数一覧から読む(引数一覧) {
        Ok(モード) => モード,
        Err(知らない語) => {
            eprintln!("[xtask] clean-build-cacheが知らない引数を受け取った: {知らない語}(受け取るのは{消さずに一覧だけ出す旗}だけである)");
            return ExitCode::FAILURE;
        }
    };
    match 掃除する(モード) {
        Ok(解放した容量) => {
            println!("[xtask] clean-build-cache成功: {}{解放した容量}", モード.合計の見出し());
            ExitCode::SUCCESS
        }
        Err(破れ) => {
            eprintln!("[xtask] clean-build-cache失敗: {破れ}");
            ExitCode::FAILURE
        }
    }
}

fn 掃除する(モード: 実行モード) -> Result<占める容量, 掃除の破れ> {
    let mut 合計 = 占める容量::無し();
    for 候補 in collect::掃除の候補を集める()? {
        合計 = 合計.足す(候補.報告して処理する(モード)?);
    }
    Ok(合計)
}
