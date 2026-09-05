//! ビルドの中間データの掃除の入口。差分ビルドの中間データ・codexレビュー用のビルドの出力先・
//! 作業ツリーごとのビルドの出力先を、実在するものだけ消し、消した対象と解放した容量を1行ずつ表示する。
//!
//! Cargoは条件が変わって別の単位になった中間データを消さないため、掃除の仕組みがCargo自体に無い。
//! 開発者が消さない限り増え続け、3日の作業で500件を超えた実測がある。
//! 参照: [Issue #61](https://github.com/megaDenryu/Blitzdrache0/issues/61)。

mod collect;
mod error;
mod occupied_size;
mod subject;

use std::process::ExitCode;

use collect::掃除の候補;
use error::掃除の破れ;
use occupied_size::占める容量;

/// 消さずに一覧だけを出す旗の綴り。
const 消さずに一覧だけ出す旗: &str = "--dry-run";

pub(crate) fn ビルドの中間データを掃除する(引数一覧: &[String]) -> ExitCode {
    let 消さずに一覧だけ出すか = match 旗を読む(引数一覧) {
        Ok(旗) => 旗,
        Err(知らない語) => {
            eprintln!("[xtask] clean-build-cacheが知らない引数を受け取った: {知らない語}(受け取るのは{消さずに一覧だけ出す旗}だけである)");
            return ExitCode::FAILURE;
        }
    };
    match 掃除する(消さずに一覧だけ出すか) {
        Ok(解放した容量) => {
            println!("[xtask] clean-build-cache成功: {}{解放した容量}", 見出しの接頭(消さずに一覧だけ出すか));
            ExitCode::SUCCESS
        }
        Err(破れ) => {
            eprintln!("[xtask] clean-build-cache失敗: {破れ}");
            ExitCode::FAILURE
        }
    }
}

fn 旗を読む(引数一覧: &[String]) -> Result<bool, &str> {
    let mut 消さずに一覧だけ出すか = false;
    for 語 in 引数一覧 {
        if 語 == 消さずに一覧だけ出す旗 {
            消さずに一覧だけ出すか = true;
            continue;
        }
        return Err(語);
    }
    Ok(消さずに一覧だけ出すか)
}

fn 見出しの接頭(消さずに一覧だけ出すか: bool) -> &'static str {
    if 消さずに一覧だけ出すか {
        "消せば解放する容量の合計は"
    } else {
        "解放した容量の合計は"
    }
}

fn 掃除する(消さずに一覧だけ出すか: bool) -> Result<占める容量, 掃除の破れ> {
    let mut 合計 = 占める容量::無し();
    for 候補 in collect::掃除の候補を集める()? {
        合計 = 合計.足す(候補1つを処理する(&候補, 消さずに一覧だけ出すか)?);
    }
    Ok(合計)
}

/// 候補1件を報告し、消す枝で実在するものだけを消す。返すのはその1件が解放する容量である。
fn 候補1つを処理する(候補: &掃除の候補, 消さずに一覧だけ出すか: bool) -> Result<占める容量, 掃除の破れ> {
    let (掃除の候補::消す(対象) | 掃除の候補::いま使っているため残す(対象)) = 候補;
    let 置き場 = 対象.パス().display();
    if !対象.実在するか() {
        println!("[xtask] 無い: {} ({置き場})", 対象.役割の呼び名());
        return Ok(占める容量::無し());
    }
    let 容量 = 対象.容量を測る()?;
    match 候補 {
        掃除の候補::いま使っているため残す(_) => {
            println!(
                "[xtask] 残す: {} ({置き場}, {容量})。いまのビルドがこの出力先を使っている",
                対象.役割の呼び名()
            );
            Ok(占める容量::無し())
        }
        掃除の候補::消す(_) if 消さずに一覧だけ出すか => {
            println!("[xtask] 消せる: {} ({置き場}, {容量})", 対象.役割の呼び名());
            Ok(容量)
        }
        掃除の候補::消す(_) => {
            対象.消す()?;
            println!("[xtask] 消した: {} ({置き場}, {容量})", 対象.役割の呼び名());
            Ok(容量)
        }
    }
}
