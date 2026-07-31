//! 4本のコンピュートパイプラインの生成と、途中で失敗したときの巻き戻し。担当する工程は「ディスクリプタの
//! レイアウトとシェーダーを受け取り、4本そろったパイプラインを返す」ことである。
//! 触れるのはパイプラインのハンドルだけであり、ディスクリプタは1つも作らない。

use super::descriptors::ディスクリプタ四点;
use crate::error::レンダラーエラー;
use crate::shader_bundle::大気のベイク済み画像のシェーダー一式;
use crate::vulkan::atmosphere_lut::pipeline::{即時定数の枠, 生成パイプライン};

/// スカイビュー生成が押し込む定数のバイト数。`shaders/atmosphere_skyview.slang`の`SkyViewCondition`(float2つ)と一致させる。
const スカイビュー押し込みバイト数: u32 = 8;
/// 空中遠近生成が押し込む定数のバイト数。`shaders/atmosphere_aerial.slang`の`AerialCondition`(float4が4つ)と一致させる。
/// Vulkanが保証する即時定数の下限は128バイトであり、この64バイトはその内側にある。
const 空中遠近押し込みバイト数: u32 = 64;

pub(super) fn 作る(
    device: &ash::Device,
    ディスクリプタ: &ディスクリプタ四点,
    シェーダー: &大気のベイク済み画像のシェーダー一式,
) -> Result<[生成パイプライン; 4], レンダラーエラー> {
    let 仕様一覧 = [
        (ディスクリプタ.透過率.layout, 即時定数の枠::無し, シェーダー.透過率.コード()),
        (ディスクリプタ.多重散乱.layout, 即時定数の枠::無し, シェーダー.多重散乱.コード()),
        (
            ディスクリプタ.スカイビュー.layout,
            即時定数の枠::バイト数(スカイビュー押し込みバイト数),
            シェーダー.スカイビュー.コード(),
        ),
        (
            ディスクリプタ.空中遠近.layout,
            即時定数の枠::バイト数(空中遠近押し込みバイト数),
            シェーダー.空中遠近.コード(),
        ),
    ];
    let mut 作った: Vec<生成パイプライン> = Vec::with_capacity(仕様一覧.len());
    for (layout, 押し込み, コード) in 仕様一覧 {
        match 生成パイプライン::生成する(device, layout, 押し込み, コード) {
            Ok(パイプライン) => 作った.push(パイプライン),
            Err(誤り) => {
                while let Some(パイプライン) = 作った.pop() {
                    パイプライン.破棄する(device);
                }
                return Err(誤り);
            }
        }
    }
    let 件数 = 作った.len();
    Ok(作った
        .try_into()
        .unwrap_or_else(|_| panic!("生成パイプラインを4本要求したのに{件数}本できた")))
}
