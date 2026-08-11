//! 1条件ぶんの実行と判定をひとまとめに通す工程。受け取るのは出力先と条件の指定、返すのは板の行と判定の代表値である。
//! 起動は`run`、行の取り出しは`parse`、合否は`judgment`が担い、この工程はその3つを順に通すことだけを担う。

use std::path::Path;

use super::run::注入の指定;
use super::{band, judgment, parse, run};

/// 1条件ぶんの判定結果。
pub(super) struct 条件の結果 {
    pub(super) 行一覧: Vec<parse::板の行>,
    pub(super) 最大符号値差: f64,
    pub(super) 板数: usize,
}

pub(super) fn 条件を判定する(出力先: &Path, 実行名: &str, 条件: &str, 一日内秒: &str) -> Result<条件の結果, String> {
    let 結果 = run::描画する(出力先, 実行名, 一日内秒, &注入の指定::注入する(条件), false)?;
    結果.報告().検証層の指摘が零件であることを確かめる()?;
    let 行一覧 = parse::板の行を取り出す(結果.報告().本文())?;
    let 判定 = judgment::期待と実測の一致を検査する(実行名, &行一覧)?;
    Ok(条件の結果 {
        行一覧,
        最大符号値差: 判定.最大符号値差,
        板数: 判定.板数,
    })
}

pub(super) fn 破綻防止帯を測る(出力先: &Path, 昼の一日内秒: &str) -> Result<band::帯の実測, String> {
    let 結果 = run::描画する(出力先, "band", 昼の一日内秒, &注入の指定::大気から焼く, true)?;
    結果.報告().検証層の指摘が零件であることを確かめる()?;
    band::破綻防止帯を検査する(結果.画像())
}
