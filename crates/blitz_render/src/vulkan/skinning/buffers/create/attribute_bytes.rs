//! ジョイント参照とウェイトをGPU転送用バイト列へ直列化する。

use crate::skin_mesh::スキンメッシュ素材;

pub(super) fn 属性をバイト列にする(素材: &スキンメッシュ素材) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(素材.属性一覧().len() * 32);
    for 属性 in 素材.属性一覧() {
        for 参照 in 属性.ジョイント {
            バイト列.extend_from_slice(&参照.to_le_bytes());
        }
        for 重み in 属性.ウェイト {
            バイト列.extend_from_slice(&重み.to_le_bytes());
        }
    }
    バイト列
}
