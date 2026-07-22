//! 粒子系GPUパスの初期状態を、起動時に選ばれた検証対象から構築する。

use blitz_render::粒子素材;

use crate::cli::粒子表示モード;
use crate::error::起動エラー;

const 粒子トイ要素数: u32 = 4096;
const 表面流一辺セル数: u32 = 128;
const 要素バイト数: usize = 32;

pub(super) fn 素材を作る(表示: 粒子表示モード) -> Result<Option<粒子素材>, 起動エラー> {
    match 表示 {
        粒子表示モード::なし => Ok(None),
        粒子表示モード::粒子トイ => Ok(Some(粒子トイ素材を作る()?)),
        粒子表示モード::表面流 => Ok(Some(表面流素材を作る()?)),
        粒子表示モード::Sph512 => Ok(Some(super::sph_setup::素材を作る(512)?)),
        粒子表示モード::Sph1024 => Ok(Some(super::sph_setup::素材を作る(1024)?)),
        粒子表示モード::Sph2048 => Ok(Some(super::sph_setup::素材を作る(2048)?)),
    }
}

fn 表面流素材を作る() -> Result<粒子素材, 起動エラー> {
    let 仕様 = blitz_sim::表面流仕様::生成する([表面流一辺セル数; 2], 1.0, 1.0 / 60.0, [0.0, -1.5], 0.99)?;
    let 状態 = blitz_sim::表面流状態::液膜分布で生成する(
        &仕様,
        |[列, 行]| {
            if (42..86).contains(&列) && (84..116).contains(&行) { 1.0 } else { 0.0 }
        },
    )?;
    let 要素数 = 表面流一辺セル数 * 表面流一辺セル数;
    Ok(粒子素材::生成する(blitz_sim::表面流バイト列にする(&状態), 要素数)?)
}

fn 粒子トイ素材を作る() -> Result<粒子素材, 起動エラー> {
    let 件数 = usize::try_from(粒子トイ要素数).unwrap_or_else(|_| panic!("u32がusizeへ変換できないプラットフォームは対象外"));
    let mut バイト列 = Vec::with_capacity(件数 * 要素バイト数);
    let mut 乱数状態 = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..件数 {
        let (x, y, z) = 次の球面座標を作る(&mut 乱数状態);
        for 成分 in [x, y, z, 0.0, 0.0, 0.0, 0.0, 0.0] {
            バイト列.extend_from_slice(&成分.to_le_bytes());
        }
    }
    Ok(粒子素材::生成する(バイト列, 粒子トイ要素数)?)
}

fn 次の球面座標を作る(乱数状態: &mut u64) -> (f32, f32, f32) {
    let 経度 = 次の一様乱数を作る(乱数状態) * std::f32::consts::TAU;
    let 余弦緯度 = 次の一様乱数を作る(乱数状態) * 2.0 - 1.0;
    let 正弦緯度 = (1.0 - 余弦緯度 * 余弦緯度).max(0.0).sqrt();
    (2.0 * 正弦緯度 * 経度.cos(), 2.0 * 余弦緯度, 2.0 * 正弦緯度 * 経度.sin())
}

fn 次の一様乱数を作る(乱数状態: &mut u64) -> f32 {
    *乱数状態 = 乱数状態.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *乱数状態;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let ビット = u32::try_from(z & 0xFFFF_FFFF).unwrap_or_else(|_| panic!("マスク済みの値がu32に収まらない"));
    f32::from_bits((127u32 << 23) | (ビット >> 9)) - 1.0
}
