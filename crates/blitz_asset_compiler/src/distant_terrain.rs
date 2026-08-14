//! 世界の高さ場から25メートル格子の遠景メッシュを焼く入口。

mod deviation;
mod error;
mod grid;
mod mesh;
mod scene;
mod statistics;
#[cfg(test)]
mod tests;

use std::path::PathBuf;

use crate::asset_layout::チャンクの高さ格子ソース;
use crate::compile::コンパイル済みシーン;

pub use error::遠景コンパイルエラー;
pub use statistics::遠景の沈み統計;

pub struct コンパイル済み遠景 {
    pub シーン: コンパイル済みシーン,
    pub 沈み統計: 遠景の沈み統計,
}

pub fn 遠景アセットをコンパイルする(
    ソース一覧: &[チャンクの高さ格子ソース],
    安全幅メートル: f32,
) -> Result<コンパイル済み遠景, 遠景コンパイルエラー> {
    let (高さ場, 依存一覧) = 高さ場を作る(ソース一覧)?;
    let 格子 = grid::遠景格子::高さ場から作る(高さ場)?;
    let 偏差 = deviation::セルごとの最大正偏差を求める(&格子)?;
    let 沈み = deviation::頂点ごとの沈みを求める(&格子, &偏差, 安全幅メートル)?;
    let 統計 = statistics::統計を求める(&格子, &偏差, &沈み, 安全幅メートル)?;
    let メッシュ = mesh::沈めたメッシュを作る(&格子, &沈み)?;
    let シーンデータ = scene::通常シーンを作る(メッシュ, 依存一覧);
    Ok(コンパイル済み遠景 {
        シーン: コンパイル済みシーン::シーンデータから焼く(シーンデータ)?,
        沈み統計: 統計,
    })
}

fn 高さ場を作る(
    ソース一覧: &[チャンクの高さ格子ソース],
) -> Result<(blitz_engine::height_field::高さ場, Vec<PathBuf>), 遠景コンパイルエラー> {
    let mut 格子一覧 = Vec::with_capacity(ソース一覧.len());
    let mut 依存一覧 = Vec::with_capacity(ソース一覧.len());
    for ソース in ソース一覧 {
        格子一覧.push(ソース.読み込む()?);
        依存一覧.push(ソース.依存一覧へ載せるパス());
    }
    Ok((crate::height_field::チャンクごとの高さ格子から高さ場を組み立てる(&格子一覧)?, 依存一覧))
}
