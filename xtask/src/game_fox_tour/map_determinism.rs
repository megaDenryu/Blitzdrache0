//! 同じ乱数の種から同じマップが出ることの検査。受け取るのは種、返すのは突き合わせた本数の要約である。
//! 種からソースアセットを2度作り、1度目のバイト列を覚えてから2度目と突き合わせる。
//!
//! ソースアセット(高さ格子と目印の文書)と実行時形式(地形チャンクと高さ場とカタログ)の両方を見るのは、
//! 決定性が破れうる場所が2つあるためである。1つは種から高さを決める計算であり、もう1つは高さ格子から
//! メッシュと高さ場を焼く工程である。片方だけを見ると、もう片方の揺れが絵の食い違いとして遠くで現れる。

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// 突き合わせるディレクトリ。ソースアセットの置き場と、実行時形式の置き場である。
const 突き合わせるディレクトリ一覧: [&str; 2] = ["assets/fox_tour_world", "target/fox_tour_assets"];

pub(super) fn 同じ種から同じマップが出ることを確かめる(種: &str) -> Result<String, String> {
    if !crate::gen_game_map::生成する(種) {
        return Err(format!("種{種}からのマップ生成に失敗した(1度目)"));
    }
    let 一度目 = 全ファイルのバイト列を読む()?;
    if !crate::gen_game_map::生成する(種) {
        return Err(format!("種{種}からのマップ生成に失敗した(2度目)"));
    }
    let 二度目 = 全ファイルのバイト列を読む()?;
    突き合わせる(&一度目, &二度目)?;
    Ok(format!("種{種}から2度作ったマップが{}本のファイルでバイト一致した", 一度目.len()))
}

fn 突き合わせる(一度目: &BTreeMap<PathBuf, Vec<u8>>, 二度目: &BTreeMap<PathBuf, Vec<u8>>) -> Result<(), String> {
    if 一度目.len() != 二度目.len() {
        return Err(format!("2度の生成でファイルの本数が食い違った({}本と{}本)", 一度目.len(), 二度目.len()));
    }
    for (パス, 一度目のバイト列) in 一度目 {
        let Some(二度目のバイト列) = 二度目.get(パス) else {
            return Err(format!("2度目の生成に{}が無い", パス.display()));
        };
        if 一度目のバイト列 != 二度目のバイト列 {
            return Err(format!("2度の生成で{}のバイト列が食い違った", パス.display()));
        }
    }
    Ok(())
}

fn 全ファイルのバイト列を読む() -> Result<BTreeMap<PathBuf, Vec<u8>>, String> {
    let mut 表 = BTreeMap::new();
    for ディレクトリ in 突き合わせるディレクトリ一覧 {
        ディレクトリのファイルを読む(Path::new(ディレクトリ), &mut 表)?;
    }
    if 表.is_empty() {
        return Err("マップの生成物が1本も見つからなかった".to_string());
    }
    Ok(表)
}

fn ディレクトリのファイルを読む(ディレクトリ: &Path, 表: &mut BTreeMap<PathBuf, Vec<u8>>) -> Result<(), String> {
    let 一覧 = std::fs::read_dir(ディレクトリ).map_err(|誤り| format!("{}を開けなかった: {誤り}", ディレクトリ.display()))?;
    for 項目 in 一覧 {
        let 項目 = 項目.map_err(|誤り| format!("{}の走査に失敗した: {誤り}", ディレクトリ.display()))?;
        let パス = 項目.path();
        if !パス.is_file() {
            continue;
        }
        let バイト列 = std::fs::read(&パス).map_err(|誤り| format!("{}を読めなかった: {誤り}", パス.display()))?;
        表.insert(パス, バイト列);
    }
    Ok(())
}
