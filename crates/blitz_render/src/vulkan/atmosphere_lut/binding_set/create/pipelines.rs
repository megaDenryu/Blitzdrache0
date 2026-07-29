//! 3本のコンピュートパイプラインの生成と、途中で失敗したときの巻き戻し。担当する工程は「ディスクリプタの
//! レイアウトとシェーダーを受け取り、3本そろったパイプラインを返す」ことである。
//! 触れるのはパイプラインのハンドルだけであり、ディスクリプタは1つも作らない。

use super::descriptors::ディスクリプタ三点;
use crate::error::レンダラーエラー;
use crate::shader_bundle::大気LUTシェーダー一式;
use crate::vulkan::atmosphere_lut::pipeline::{押し込み定数の枠, 生成パイプライン};

/// スカイビュー生成が押し込む定数のバイト数。`shaders/atmosphere_skyview.slang`の`SkyViewCondition`(float2つ)と一致させる。
const スカイビュー押し込みバイト数: u32 = 8;

pub(super) fn 作る(
    device: &ash::Device,
    ディスクリプタ: &ディスクリプタ三点,
    シェーダー: &大気LUTシェーダー一式,
) -> Result<[生成パイプライン; 3], レンダラーエラー> {
    let 仕様一覧 = [
        (ディスクリプタ.透過率.layout, 押し込み定数の枠::無し, シェーダー.透過率.コード()),
        (ディスクリプタ.多重散乱.layout, 押し込み定数の枠::無し, シェーダー.多重散乱.コード()),
        (
            ディスクリプタ.スカイビュー.layout,
            押し込み定数の枠::バイト数(スカイビュー押し込みバイト数),
            シェーダー.スカイビュー.コード(),
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
        .unwrap_or_else(|_| panic!("生成パイプラインを3本要求したのに{件数}本できた")))
}
