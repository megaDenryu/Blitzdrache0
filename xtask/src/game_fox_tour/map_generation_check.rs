//! 種からのマップ生成が満たすべき2つの性質の検収。受け取るのは種、返すのは要約である。
//! クリーンな2つの生成がバイト一致することは`determinism`が、同じ種での再実行が1本も焼き直さないことは
//! `incremental`が確かめる。ファイルの畳み込みは`file_digest`、報告の行の読み取りは`tally_line`が持つ。
//!
//! 増分・決定性・続く撮影はすべて検収専用ルートを使い、遊ぶ世界とリポジトリ内のソースへ書かない。
//! 参照: `_doc/設計/大規模世界の生成と遠景.md`

mod check_root;
mod determinism;
mod file_digest;
mod incremental;
mod tally_line;

use blitz_asset_compiler::{マップ生成の乱数の種, 世界の広がり};

use super::error::場所巡りの通しの検収エラー;

pub(super) fn 種からの生成を確かめる(種: マップ生成の乱数の種) -> Result<String, 場所巡りの通しの検収エラー> {
    確かめる(種, 世界の広がり::場所巡りの既定値(), check_root::生成検収の親ディレクトリ::場所巡り)
}

pub(crate) fn 大規模世界を確かめる(
    種: マップ生成の乱数の種,
    広がり: 世界の広がり,
) -> Result<String, 場所巡りの通しの検収エラー> {
    let 要約 = 確かめる(種, 広がり, check_root::生成検収の親ディレクトリ::大規模世界)?;
    check_root::大規模世界の計測入力以外を掃除する()?;
    Ok(要約)
}

fn 確かめる(
    種: マップ生成の乱数の種,
    広がり: 世界の広がり,
    親: check_root::生成検収の親ディレクトリ,
) -> Result<String, 場所巡りの通しの検収エラー> {
    let 増分 = incremental::同じ種での再実行が焼き直さないことを確かめる(種, 広がり, 親)?;
    let 決定性 = determinism::同じ種から同じバイト列が出ることを確かめる(種, 広がり, 親)?;
    Ok(format!("{決定性}、{増分}"))
}

pub(super) fn 撮影用の実行時アセットルート() -> crate::acceptance::実行時アセットルート {
    check_root::検収用のルート::実行時形式の一度目(check_root::生成検収の親ディレクトリ::場所巡り).実行時アセットルートとして読む()
}

pub(crate) fn 大規模世界の計測入力パス() -> std::path::PathBuf {
    check_root::大規模世界の計測入力パス()
}
