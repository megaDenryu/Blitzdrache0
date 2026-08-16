//! 世界の高さ場から25メートル格子の遠景メッシュを焼く入口。

mod cell_corner;
mod cell_scan;
mod deviation;
mod error;
mod gap;
mod grid;
mod grid_measure;
mod mesh;
mod scene;
mod sink;
mod statistics;
mod subdivision_point;
#[cfg(test)]
mod tests;
mod vertex;

use std::path::PathBuf;

use blitz_math::メートル;

use crate::assembled_scene::組み立てたシーン;
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
    安全幅: メートル,
) -> Result<コンパイル済み遠景, 遠景コンパイルエラー> {
    let (高さ場, 依存一覧) = 高さ場を作る(ソース一覧)?;
    let 格子 = grid::遠景格子::高さ場から作る(高さ場)?;
    let 走査 = cell_scan::細分点の走査::生成する(&格子)?;
    let 偏差 = 走査.セルごとの最大正偏差を求める()?;
    let 沈み = sink::頂点ごとの沈み::求める(&格子, &偏差, 安全幅)?;
    let 隙間 = 走査.沈めた面と詳細面の隙間を調べる(&沈み)?;
    let 統計 = 遠景の沈み統計::求める(&偏差, &隙間, 安全幅)?;
    let メッシュ = mesh::沈めたメッシュを作る(&格子, &沈み)?;
    // 遠景地形は1枚の通常メッシュであり、インスタンス群を1つも持たない。
    let 組み立て = 組み立てたシーン::群を持たないシーンから生成する(scene::通常シーンを作る(メッシュ, 依存一覧));
    Ok(コンパイル済み遠景 {
        シーン: コンパイル済みシーン::組み立てたシーンから焼く(組み立て)?,
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
