//! 保存済み対照の先頭行を読み、比較対象の深度契約と遠クリップが一致することを確かめる工程。

use std::path::PathBuf;

use super::super::error::逆Z検収エラー;
use crate::verify::検証の出力ルート;

const 期待する対照の深度契約: &str = "depth-contract=standard-z far=10000";

pub(super) fn 対照の深度契約を検査する() -> Result<(), 逆Z検収エラー> {
    let パス = 検証の出力ルート::既定().置き場の中のファイル(super::super::出力ディレクトリ, super::super::対照の由来ファイル名);
    let 内容 = std::fs::read_to_string(&パス).map_err(|誤り| 由来を読めない誤りへ写す(パス.clone(), 誤り))?;
    対照の深度契約の先頭行を検査する(&内容)
}

fn 対照の深度契約の先頭行を検査する(内容: &str) -> Result<(), 逆Z検収エラー> {
    let 実際 = 内容.lines().next().unwrap_or("");
    if 実際 != 期待する対照の深度契約 {
        return Err(逆Z検収エラー::対照の深度契約が違う {
            期待: 期待する対照の深度契約,
            実際: 実際.to_string(),
        });
    }
    Ok(())
}

fn 由来を読めない誤りへ写す(パス: PathBuf, 誤り: std::io::Error) -> 逆Z検収エラー {
    if 誤り.kind() == std::io::ErrorKind::NotFound {
        逆Z検収エラー::対照の由来が無い(パス)
    } else {
        逆Z検収エラー::対照の由来を読めなかった { パス, 誤り }
    }
}

#[cfg(test)]
mod tests {
    use super::対照の深度契約の先頭行を検査する;

    #[test]
    fn 遠クリップが違う対照を拒む() {
        assert!(対照の深度契約の先頭行を検査する("depth-contract=standard-z far=2000\ncommit=x").is_err());
    }

    #[test]
    fn 期待する契約の対照を受け入れる() {
        assert!(対照の深度契約の先頭行を検査する("depth-contract=standard-z far=10000\ncommit=x").is_ok());
    }
}
