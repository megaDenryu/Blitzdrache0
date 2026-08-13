//! 大規模世界の生成検収後に、計測入力だけを残して他の使い捨てルートを掃除する。

use std::path::PathBuf;

use super::{検収用のルート, 生成検収の親ディレクトリ};
use crate::game_fox_tour::error::場所巡りの通しの検収エラー;

pub(in crate::game_fox_tour::map_generation_check) fn 大規模世界の計測入力パス() -> PathBuf {
    検収用のルート::実行時形式の一度目(生成検収の親ディレクトリ::大規模世界)
        .プロセスへ渡すパス()
        .to_path_buf()
}

pub(in crate::game_fox_tour::map_generation_check) fn 大規模世界の計測入力以外を掃除する() -> Result<(), 場所巡りの通しの検収エラー> {
    let 親 = 生成検収の親ディレクトリ::大規模世界;
    for ルート in [
        検収用のルート::ソースの一度目(親),
        検収用のルート::ソースの二度目(親),
        検収用のルート::実行時形式の二度目(親),
        検収用のルート::増分のソース(親),
        検収用のルート::増分の実行時形式(親),
    ] {
        ルート.掃除する()?;
    }
    Ok(())
}
