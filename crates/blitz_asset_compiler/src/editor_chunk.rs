//! エディターが書いた版付きチャンクソースから、地面と部品建物の群と散布の群を同じチャンクへ焼く。

mod building_definitions;
mod building_groups;
mod ground_material;
mod identifier_seed;
mod scatter_groups;
mod scene;
mod source;

use blitz_engine::{アセットID, チャンク座標};

use crate::compile::コンパイル済みシーン;
use crate::error::アセットコンパイルエラー;
use crate::scene_compiler::ソースアセットのコンパイル係;

pub(crate) use source::エディターチャンクソース;

impl ソースアセットのコンパイル係<'_> {
    pub(crate) fn エディターチャンクをコンパイルする(
        &self,
        id: &アセットID,
        所有チャンク: チャンク座標,
    ) -> Result<コンパイル済みシーン, アセットコンパイルエラー> {
        let マニフェストパス = self.安定idが指すソースのパスを参照する(id)?;
        let ソース = エディターチャンクソース::ファイルから読む(マニフェストパス)?;
        let 組み立て = self.エディターチャンクのシーンを組み立てる(&ソース, 所有チャンク, マニフェストパス.to_path_buf())?;
        コンパイル済みシーン::組み立てたシーンから焼く(組み立て)
    }
}
