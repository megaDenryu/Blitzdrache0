//! 粒子ストレージバッファの初期データ(位置vec4+速度vec4/粒子)を決定的シードで生成する。
//! 位置は球状ランダム配置、速度はゼロ(参照: `_doc/設計/レンダーグラフ.md`「GPU粒子トイ」)。

/// 粒子数(判断29)。
pub(crate) const 粒子数: u32 = 4096;
const 粒子バイト長: usize = 32;
const 球の半径: f32 = 2.0;
const 決定的シード: u64 = 0x9E37_79B9_7F4A_7C15;

pub(crate) fn 初期バイト列を生成する() -> Vec<u8> {
    let 粒子数usize =
        usize::try_from(粒子数).unwrap_or_else(|_| panic!("粒子数がusizeに収まらない: {粒子数}"));
    let mut バイト列 = Vec::with_capacity(粒子数usize * 粒子バイト長);
    let mut 乱数状態 = 決定的シード;

    for _ in 0..粒子数usize {
        let (x, y, z) = 次の球面座標を生成する(&mut 乱数状態);
        for 成分 in [x, y, z, 0.0f32] {
            バイト列.extend_from_slice(&成分.to_le_bytes());
        }
        for _ in 0..4 {
            バイト列.extend_from_slice(&0.0f32.to_le_bytes());
        }
    }
    バイト列
}

/// 球面上の一様分布に近い座標を生成する(初速ゼロのため速度は呼び出し元がゼロ埋めする)。
fn 次の球面座標を生成する(乱数状態: &mut u64) -> (f32, f32, f32) {
    let u = 次の一様乱数を生成する(乱数状態);
    let v = 次の一様乱数を生成する(乱数状態);
    let 経度 = u * std::f32::consts::TAU;
    let 余弦緯度 = v * 2.0 - 1.0;
    let 正弦緯度 = (1.0 - 余弦緯度 * 余弦緯度).max(0.0).sqrt();
    (
        球の半径 * 正弦緯度 * 経度.cos(),
        球の半径 * 余弦緯度,
        球の半径 * 正弦緯度 * 経度.sin(),
    )
}

/// splitmix64で乱数状態を更新し、上位23bitをf32仮数部に詰めて[0,1)の値を作る
/// (`as`キャストを使わず、常に成功するマスク済みu32へのTryFromのみで構成する)。
fn 次の一様乱数を生成する(乱数状態: &mut u64) -> f32 {
    *乱数状態 = 乱数状態.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *乱数状態;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;

    let 下位32bit = z & 0xFFFF_FFFF;
    let ビット = u32::try_from(下位32bit)
        .unwrap_or_else(|_| panic!("0xFFFF_FFFFでマスク済みの値がu32に収まらない(実装のバグ)"));
    let 仮数部 = ビット >> 9;
    let 単精度ビット表現 = (127u32 << 23) | 仮数部;
    f32::from_bits(単精度ビット表現) - 1.0
}
