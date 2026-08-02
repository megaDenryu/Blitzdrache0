//! 頂点量の診断用に面を格子へ細分化した直方体の頂点バイト列。担当するのは、面ごとの格子を
//! 位置→法線→接線→UV→インデックスの区間へ並べてバイト化することである。
//! 面の張り方と格子の頂点そのものは`faces`が持つ。
//!
//! 区間の並び順は粗い原型と同じであり、`gltf_json`のbufferViewsへ`stage_amount`の量を通して一致させている。
//! 参照: `crates/blitz_asset_compiler/examples/generate_source_assets/vegetation_world/geometry.rs`

use super::super::geometry::直方体諸元;
use super::super::stage_amount::段の中身の量;
use super::faces;

/// 1つの格子は三角形2枚になる。
const 格子のインデックス数: usize = 6;

/// 細分化した直方体1つぶんの中身の量。段が違っても分割数は同じであるため、どの段もこの量である。
pub(super) const 直方体の量: 段の中身の量 = 段の中身の量 {
    頂点数: faces::面数 * faces::面の頂点数,
    インデックス数: faces::面数 * faces::面の格子数 * 格子のインデックス数,
};

pub(super) fn バッファバイト列を作る(諸元一覧: &[直方体諸元]) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(直方体の量.バイト長() * 諸元一覧.len());
    for 諸元 in 諸元一覧 {
        直方体を並べる(&mut バイト列, *諸元);
    }
    バイト列
}

fn 直方体を並べる(バイト列: &mut Vec<u8>, 諸元: 直方体諸元) {
    let 面一覧 = faces::面一覧を作る(諸元);
    for 面 in &面一覧 {
        小数を並べる(バイト列, 面.位置一覧().into_iter().flatten());
    }
    for 面 in &面一覧 {
        小数を並べる(バイト列, 面.法線.iter().copied().cycle().take(faces::面の頂点数 * 面.法線.len()));
    }
    for 面 in &面一覧 {
        小数を並べる(バイト列, 面.接線.iter().copied().cycle().take(faces::面の頂点数 * 面.接線.len()));
    }
    for _ in 0..faces::面数 {
        小数を並べる(バイト列, faces::テクスチャ座標一覧().into_iter().flatten());
    }
    インデックスを並べる(バイト列);
}

/// 格子1つを2枚の三角形へ割る。回り順は粗い原型の四角形と同じ(左下→右下→右上、左下→右上→左上)である。
fn インデックスを並べる(バイト列: &mut Vec<u8>) {
    let 一辺 = 添字へ変換する(faces::一辺の頂点数);
    let 面の頂点数 = 添字へ変換する(faces::面の頂点数);
    for 面番号 in 0..添字へ変換する(faces::面数) {
        let 基点 = 面番号 * 面の頂点数;
        for 行 in 0..一辺 - 1 {
            for 列 in 0..一辺 - 1 {
                let 左下 = 基点 + 行 * 一辺 + 列;
                for 相対 in [0, 1, 一辺 + 1, 0, 一辺 + 1, 一辺] {
                    バイト列.extend_from_slice(&(左下 + 相対).to_le_bytes());
                }
            }
        }
    }
}

/// glTFのインデックスはu16である。分割数を上げてu16を超える形にしたときに、静かに折り返さず落ちるようにする。
fn 添字へ変換する(値: usize) -> u16 {
    u16::try_from(値).unwrap_or_else(|_| panic!("細分化した原型の頂点の添字がu16に収まらない: {値}"))
}

fn 小数を並べる(バイト列: &mut Vec<u8>, 成分一覧: impl Iterator<Item = f32>) {
    for 成分 in 成分一覧 {
        バイト列.extend_from_slice(&成分.to_le_bytes());
    }
}
