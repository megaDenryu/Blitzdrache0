//! 遠方環境の検収世界の頂点データをglTFのバッファバイト列へ直列化する工程。受け取るのは板の一覧、
//! 返すのは.binの中身と位置の値域である。
//!
//! 区間の並び順(位置→法線→接線→UV→インデックス)と各区間のバイト長は`indirect_probe_gltf_json`の
//! bufferViewsへ手で一致させている(材質境界の検収アセットと同じ作り)。

use super::indirect_probe_plates::{一覧, 半辺, 板の宣言, 面内の軸};

/// 板1枚あたりの頂点数と、板の枚数から決まる各区間のバイト長。
const 板の頂点数: usize = 4;
pub(super) const 板の枚数: usize = 8;
pub(super) const 位置のバイト長: usize = 板の枚数 * 板の頂点数 * 12;
pub(super) const 法線のバイト長: usize = 位置のバイト長;
pub(super) const 接線のバイト長: usize = 板の枚数 * 板の頂点数 * 16;
pub(super) const テクスチャ座標のバイト長: usize = 板の枚数 * 板の頂点数 * 8;
pub(super) const インデックスのバイト長: usize = 板の枚数 * 6 * 2;

pub(super) fn バッファバイト列を作る() -> Vec<u8> {
    let 板一覧 = 一覧();
    let mut バイト列 = Vec::new();
    for 板 in &板一覧 {
        書き込む3成分(&mut バイト列, 四隅を求める(板));
    }
    for 板 in &板一覧 {
        書き込む3成分(&mut バイト列, [板.法線; 板の頂点数]);
    }
    for 板 in &板一覧 {
        let (横軸, _) = 面内の軸(板.法線);
        for _ in 0..板の頂点数 {
            for 成分 in [横軸[0], 横軸[1], 横軸[2], 1.0] {
                バイト列.extend_from_slice(&成分.to_le_bytes());
            }
        }
    }
    for _ in 0..板の枚数 {
        for 座標 in [[0.0f32, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]] {
            for 成分 in 座標 {
                バイト列.extend_from_slice(&成分.to_le_bytes());
            }
        }
    }
    for 添字 in インデックス一覧() {
        バイト列.extend_from_slice(&添字.to_le_bytes());
    }
    バイト列
}

/// 位置アクセサが要る値域。glTFは全成分の最小と最大の宣言を求める。
pub(super) fn 位置の値域() -> ([f32; 3], [f32; 3]) {
    let mut 最小 = [f32::MAX; 3];
    let mut 最大 = [f32::MIN; 3];
    for 板 in &一覧() {
        for 頂点 in 四隅を求める(板) {
            for 軸 in 0..3 {
                最小[軸] = 最小[軸].min(頂点[軸]);
                最大[軸] = 最大[軸].max(頂点[軸]);
            }
        }
    }
    (最小, 最大)
}

/// 表から見て反時計回りに並ぶ4隅。並びはUVの(0,0)→(1,0)→(1,1)→(0,1)に対応する。
fn 四隅を求める(板: &板の宣言) -> [[f32; 3]; 板の頂点数] {
    let (横軸, 縦軸) = 面内の軸(板.法線);
    let 隅 = |横符号: f32, 縦符号: f32| {
        let 成分 = |軸: usize| 板.中心[軸] + 半辺 * (横符号 * 横軸[軸] + 縦符号 * 縦軸[軸]);
        [成分(0), 成分(1), 成分(2)]
    };
    [隅(-1.0, -1.0), 隅(1.0, -1.0), 隅(1.0, 1.0), 隅(-1.0, 1.0)]
}

fn インデックス一覧() -> Vec<u16> {
    let mut 一覧 = Vec::with_capacity(板の枚数 * 6);
    for 板番号 in 0..板の枚数 {
        let 起点 = u16::try_from(板番号 * 板の頂点数).unwrap_or_else(|_| panic!("検収世界の頂点番号がu16に収まらない"));
        一覧.extend_from_slice(&[起点, 起点 + 1, 起点 + 2, 起点, 起点 + 2, 起点 + 3]);
    }
    一覧
}

fn 書き込む3成分(バイト列: &mut Vec<u8>, 値一覧: [[f32; 3]; 板の頂点数]) {
    for 値 in 値一覧 {
        for 成分 in 値 {
            バイト列.extend_from_slice(&成分.to_le_bytes());
        }
    }
}
