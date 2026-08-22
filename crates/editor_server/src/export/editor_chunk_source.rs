//! エディターチャンクの版付きソース。高さ格子と地表材質の重み格子と建物配置と建物の格子と散布の群を、
//! 1つの明示的な入口から参照させる。
//!
//! 格子を別ファイルへ出さずここへ抱えるのは、格子の正本が編集データの置き場にあって世界のソースの外に
//! あるためである。抱えることで、焼く側は世界のソースの中だけを見れば足り、増分の鍵もマニフェスト1本で済む。

use serde::Serialize;

use self::placement::建物配置ソース;
use self::scatter::散布の群ソース;
use super::error::書き出しエラー;
use blitz_asset_compiler::建物の格子ソース;

use crate::resource::{建物の配置, 建物外形カタログ, 散布の個体};

mod coordinates;
mod placement;
mod scatter;
#[cfg(test)]
mod scatter_tests;
#[cfg(test)]
mod tests;

/// 書き出す形式版。読み手はblitz_asset_compilerの`editor_chunk/source.rs`にあり、
/// 両者の版が食い違うと焼きが必ず「形式版に対応していない」で落ちる。一致は`cargo xtask conform`の定数の組が見る。
pub(super) const エディターチャンクソースの形式版: u32 = 4;

#[derive(Serialize)]
pub(super) struct エディターチャンクソース {
    形式版: u32,
    高さ格子: String,
    地表材質の重み格子: String,
    建物配置一覧: Vec<建物配置ソース>,
    建物の格子一覧: Vec<建物の格子ソース>,
    散布の群一覧: Vec<散布の群ソース>,
}

impl エディターチャンクソース {
    pub(super) fn 組み立てる(
        高さ格子: String,
        地表材質の重み格子: String,
        建物一覧: Vec<建物の配置>,
        建物の格子一覧: Vec<建物の格子ソース>,
        散布の個体一覧: Vec<散布の個体>,
        チャンク一辺メートル: f32,
        カタログ: &建物外形カタログ,
    ) -> Result<Self, 書き出しエラー> {
        let チャンク一辺 = f64::from(チャンク一辺メートル);
        let 建物配置一覧 = 建物一覧
            .into_iter()
            .map(|建物| 建物配置ソース::エディター座標から変換する(建物, チャンク一辺, カタログ))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            形式版: エディターチャンクソースの形式版,
            高さ格子,
            地表材質の重み格子,
            建物配置一覧,
            建物の格子一覧,
            散布の群一覧: 散布の群ソース::エディターの個体一覧から組み立てる(散布の個体一覧, チャンク一辺)?,
        })
    }

    pub(super) fn 整形済みjsonを作る(&self) -> Result<Vec<u8>, 書き出しエラー> {
        let mut バイト列 = serde_json::to_vec_pretty(self)?;
        バイト列.push(b'\n');
        Ok(バイト列)
    }
}
