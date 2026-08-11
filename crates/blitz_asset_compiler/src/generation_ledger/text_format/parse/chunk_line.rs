//! チャンクの行の一覧を、座標から内容ハッシュへの対応表へ読む工程。受け取るのは見出しより後ろの行の並び、
//! 返すのは座標を鍵にした対応表である。行番号を受け取るのは、誤りの報せに台帳のどの行かを載せるためである。

use std::collections::BTreeMap;

use super::super::super::content_hash::内容ハッシュ;
use super::super::super::error::生成台帳エラー;
use super::super::チャンクの欄;

/// 形式宣言と見出しが占める行数。チャンクの行の行番号はこの次から始まる。
const 見出しが占める行数: usize = 5;

pub(super) fn チャンクの行一覧を読む(行一覧: &[&str]) -> Result<BTreeMap<(i32, i32), 内容ハッシュ>, 生成台帳エラー> {
    let mut 記録 = BTreeMap::new();
    for (添字, 行) in 行一覧.iter().enumerate() {
        if 行.trim().is_empty() {
            continue;
        }
        let (座標, ハッシュ) = 一行を読む(添字 + 見出しが占める行数 + 1, 行)?;
        記録.insert(座標, ハッシュ);
    }
    Ok(記録)
}

fn 一行を読む(行番号: usize, 行: &str) -> Result<((i32, i32), 内容ハッシュ), 生成台帳エラー> {
    let 欄一覧: Vec<&str> = 行.split_whitespace().collect();
    let 行不正 = || 生成台帳エラー::行を読めない {
        行番号,
        内容: 行.to_string(),
    };
    let [名前, 東の綴り, 南の綴り, ハッシュの綴り] = 欄一覧.as_slice() else {
        return Err(行不正());
    };
    let (Ok(東), Ok(南)) = (東の綴り.parse::<i32>(), 南の綴り.parse::<i32>()) else {
        return Err(行不正());
    };
    if *名前 != チャンクの欄 {
        return Err(行不正());
    }
    Ok(((東, 南), 内容ハッシュ::十六進の綴りから復元する(ハッシュの綴り).ok_or_else(行不正)?))
}
