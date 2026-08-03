//! 読み戻したバイト列をテクセルごとに切り分けて単精度の成分へ開く工程。受け取るのはバイト列、
//! 返すのは4成分または2成分の単精度の一覧である。
//!
//! 1つの値の変換そのものは`crate::numeric::half_precision`が持つ。ここが担うのは切り分けの刻みだけであり、
//! 4成分と2成分で刻みが変わることをこのファイルが1箇所で決める。

use crate::numeric::half_precision::半精度を単精度へ;

/// 1テクセルのバイト数。R16G16B16A16_SFLOATの4成分ぶんである。
pub(super) const 四成分のテクセルのバイト数: usize = 8;

/// 1テクセルのバイト数。R16G16_SFLOATの2成分ぶんであり、反射率積分表だけが使う。
pub(super) const 二成分のテクセルのバイト数: usize = 4;

pub(super) fn 単精度へ開く(バイト列: &[u8]) -> Vec<[f32; 4]> {
    バイト列
        .chunks_exact(四成分のテクセルのバイト数)
        .map(|テクセル| {
            let mut 成分 = [0.0_f32; 4];
            for (添字, 対) in テクセル.chunks_exact(2).enumerate() {
                成分[添字] = 半精度を単精度へ(u16::from_le_bytes([対[0], 対[1]]));
            }
            成分
        })
        .collect()
}

pub(super) fn 二成分を単精度へ開く(バイト列: &[u8]) -> Vec<[f32; 2]> {
    バイト列
        .chunks_exact(二成分のテクセルのバイト数)
        .map(|テクセル| {
            let mut 成分 = [0.0_f32; 2];
            for (添字, 対) in テクセル.chunks_exact(2).enumerate() {
                成分[添字] = 半精度を単精度へ(u16::from_le_bytes([対[0], 対[1]]));
            }
            成分
        })
        .collect()
}
