//! ホスト可視バッファのマッピングと、読み戻し画像への変換(BGRA→RGBA並び替え・半精度→単精度の変換を含む)。
//!
//! 注意: 呼び出し元は、このバッファへのGPU書き込み(コピーコマンド)がフェンス待機で完了済みであることを保証すること。ここではマッピング前の同期は行わない。ホスト可視バッファのマッピングそのものは`host_bytes`が持つ。

mod host_bytes;

use ash::vk;

use super::buffer::読み戻しバッファ;
use super::target::読み戻し対象;
use crate::error::レンダラーエラー;
use crate::numeric::half_precision::半精度を単精度へ;
use crate::readback_image::{
    HDR読み戻し画像, 動きベクトル読み戻し画像, 局所可視度読み戻し画像, 深度読み戻し画像, 読み戻し画像
};

pub(crate) fn 読み取る(
    device: &ash::Device,
    バッファ: &読み戻しバッファ,
    寸法: vk::Extent2D,
    形式: vk::Format,
) -> Result<読み戻し画像, レンダラーエラー> {
    let 生データ = host_bytes::読む(device, バッファ, 寸法, 読み戻し対象::提示画像)?;
    let rgba = if bgra形式か(形式) {
        並び替える(&生データ)
    } else {
        生データ
    };
    Ok(読み戻し画像::生成する(寸法.width, 寸法.height, rgba))
}

/// 半精度4成分のバイト列を単精度の成分列へ開く。半精度の意味づけ(非正規化数・無限大・非数)を知るのはGPU境界のこの層だけであり、読み手がもう1つ復号を持つと丸めの食い違いが検収の差に化けるため、開いた形で外へ渡す。
/// 対象を引数で受けるのは、圧縮前のHDRと今のフレームの色が同じ形式・同じ並びであり、開き方を2つ持つ理由が無いためである。
pub(crate) fn 半精度四成分を読み取る(
    device: &ash::Device,
    バッファ: &読み戻しバッファ,
    寸法: vk::Extent2D,
    対象: 読み戻し対象,
) -> Result<HDR読み戻し画像, レンダラーエラー> {
    let 生データ = host_bytes::読む(device, バッファ, 寸法, 対象)?;
    let 成分 = 生データ
        .chunks_exact(2)
        .map(|対| 半精度を単精度へ(u16::from_le_bytes([対[0], 対[1]])))
        .collect();
    Ok(HDR読み戻し画像::生成する(寸法.width, 寸法.height, 成分))
}

/// 単精度1成分のバイト列を深度の列へ開く。D32_SFLOATのビット列をそのまま単精度として読むだけであり、値を丸め直さない。
pub(crate) fn 深度を読み取る(
    device: &ash::Device,
    バッファ: &読み戻しバッファ,
    寸法: vk::Extent2D,
) -> Result<深度読み戻し画像, レンダラーエラー> {
    let 生データ = host_bytes::読む(device, バッファ, 寸法, 読み戻し対象::最終深度)?;
    let 深度 = 生データ
        .chunks_exact(4)
        .map(|四つ| f32::from_le_bytes([四つ[0], 四つ[1], 四つ[2], 四つ[3]]))
        .collect();
    Ok(深度読み戻し画像::生成する(寸法.width, 寸法.height, 深度))
}

/// 8ビット無符号正規化4成分のバイト列から第1成分だけを取り出す。4成分なのは記憶画像の必須対応形式の都合であり、局所可視度が乗るのは第1成分だけである。
pub(crate) fn 局所可視度を読み取る(
    device: &ash::Device,
    バッファ: &読み戻しバッファ,
    寸法: vk::Extent2D,
) -> Result<局所可視度読み戻し画像, レンダラーエラー> {
    let 生データ = host_bytes::読む(device, バッファ, 寸法, 読み戻し対象::局所可視度)?;
    let 符号値 = 生データ.chunks_exact(4).map(|四つ| 四つ[0]).collect();
    Ok(局所可視度読み戻し画像::生成する(寸法.width, 寸法.height, 符号値))
}

/// 半精度2成分のバイト列を単精度の成分列へ開く。並びは画素ごとに横・縦の2つであり、正規化装置座標の差分そのものである。
pub(crate) fn 動きベクトルを読み取る(
    device: &ash::Device,
    バッファ: &読み戻しバッファ,
    寸法: vk::Extent2D,
) -> Result<動きベクトル読み戻し画像, レンダラーエラー> {
    let 生データ = host_bytes::読む(device, バッファ, 寸法, 読み戻し対象::動きベクトル)?;
    let 成分 = 生データ
        .chunks_exact(2)
        .map(|対| 半精度を単精度へ(u16::from_le_bytes([対[0], 対[1]])))
        .collect();
    Ok(動きベクトル読み戻し画像::生成する(寸法.width, 寸法.height, 成分))
}

fn 並び替える(データ: &[u8]) -> Vec<u8> {
    let mut 結果 = データ.to_vec();
    for 画素 in 結果.chunks_exact_mut(4) {
        画素.swap(0, 2);
    }
    結果
}

fn bgra形式か(形式: vk::Format) -> bool {
    matches!(
        形式,
        vk::Format::B8G8R8A8_UNORM | vk::Format::B8G8R8A8_SRGB | vk::Format::B8G8R8A8_SNORM | vk::Format::B8G8R8A8_UINT | vk::Format::B8G8R8A8_SINT
    )
}
