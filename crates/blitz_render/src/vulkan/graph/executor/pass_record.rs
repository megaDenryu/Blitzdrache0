//! 1パスぶんのGPU命令発行: レンダリング開始/終了の挟み込みと、パス種別ごとにGPU命令をコマンドバッファへ積むコールバックの呼び出し。

use ash::vk;

use super::super::buffer_registry::バッファレジストリ;
use super::super::context::GPU命令の積み先と宣言済み資源の取り出し口;
use super::super::pass::{パス宣言, パス種別};
use super::super::registry::画像レジストリ;
use super::super::rendering_setup;

pub(super) fn パスを記録する(
    device: &ash::Device,
    command_buffer: vk::CommandBuffer,
    画像レジストリ: &画像レジストリ,
    バッファレジストリ: &バッファレジストリ,
    パス: パス宣言<'_>,
) {
    let 宣言済み画像 = パス.宣言済み画像一覧();
    let 宣言済みバッファ = パス.宣言済みバッファ一覧();
    let 文脈 = GPU命令の積み先と宣言済み資源の取り出し口::生成する(
        device,
        command_buffer,
        画像レジストリ,
        バッファレジストリ,
        パス.名前,
        宣言済み画像,
        宣言済みバッファ,
    );

    match パス.種別 {
        パス種別::グラフィックス {
            カラー, 深度, クリア指定
        } => {
            rendering_setup::開始する(device, command_buffer, 画像レジストリ, カラー, 深度, &クリア指定);
            (パス.gpu命令をコマンドバッファへ積む)(&文脈);
            rendering_setup::終了する(device, command_buffer);
        }
        パス種別::転送 | パス種別::コンピュート => {
            (パス.gpu命令をコマンドバッファへ積む)(&文脈);
        }
    }
}
