//! 素材の段をステージングバッファ経由でGPUの画像へ積む工程。担当するのは、ホスト可視バッファの確保と解放と、
//! 選ばれた積み方に応じた一時コマンドバッファ1本ぶんの積み込みである。
//! レイアウト遷移バリアの組み立ては`barrier`、コピー領域の組み立ては`copy`に委ねる。

mod barrier;
mod copy;

use std::borrow::Cow;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::texture_material::テクスチャ素材;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::transfer::転送実行環境;

use super::create::縮小段の積み方;
use super::mip_chain;

pub(super) fn 素材の縮小段を画像へ転送する(
    確保係: &GPU資源の確保係<'_>,
    転送環境: &転送実行環境,
    image: vk::Image,
    素材: &テクスチャ素材,
    縮小段数: u32,
    積み方: 縮小段の積み方,
) -> Result<(), レンダラーエラー> {
    let 転送するバイト列 = 段を1本のバイト列へ束ねる(素材.段ごとのバイト列());
    let ステージング = 確保係.ホスト可視バッファを確保して書き込む(&転送するバイト列, vk::BufferUsageFlags::TRANSFER_SRC)?;

    let 実行結果 = 積み込んで送信する(転送環境, ステージング.バッファ(), image, 素材, 縮小段数, 積み方);

    ステージング.破棄する(確保係.論理デバイス());
    実行結果
}

/// 段が1本のときに連結しないのは、非圧縮の原寸1枚をもう一度複製しないためである。
/// 全段を運ぶ側だけがバッファ1本ぶんの連結を1回行う。
fn 段を1本のバイト列へ束ねる(段ごとのバイト列: &[Vec<u8>]) -> Cow<'_, [u8]> {
    match 段ごとのバイト列 {
        [唯一の段] => Cow::Borrowed(唯一の段),
        複数の段 => Cow::Owned(複数の段.concat()),
    }
}

fn 積み込んで送信する(
    転送環境: &転送実行環境,
    ステージングバッファ: vk::Buffer,
    image: vk::Image,
    素材: &テクスチャ素材,
    縮小段数: u32,
    積み方: 縮小段の積み方,
) -> Result<(), レンダラーエラー> {
    let 一時 = 転送環境.転送コマンドを積み始める()?;
    let device = 一時.論理デバイス();
    let command_buffer = 一時.積む先のコマンドバッファ();
    barrier::全レベルを転送先レイアウトへ遷移する(device, command_buffer, image, 縮小段数);
    match 積み方 {
        縮小段の積み方::原寸の転送とGPUのblit => {
            copy::原寸の段を画像へコピーする(device, command_buffer, ステージングバッファ, image, 素材.幅(), 素材.高さ());
            mip_chain::縮小段チェーンを積む(device, command_buffer, image, 素材.幅(), 素材.高さ(), 縮小段数);
        }
        縮小段の積み方::全段の転送 => {
            copy::全段を画像へコピーする(device, command_buffer, ステージングバッファ, image, 素材);
            barrier::全レベルをshader_readへ遷移する(device, command_buffer, image, 縮小段数);
        }
    }
    一時.送信して完了を待つ()
}
