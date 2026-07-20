//! UI頂点・インデックス列をGPU境界のバイト列へ変換する。成分ごとの`to_le_bytes`で
//! 組み立て、構造体パディングの前提に依存しない(`vulkan::geometry::bytes`と同じ方針)。

use crate::ui_vertex::UI頂点;

pub(super) fn 頂点をバイト列にする(頂点一覧: &[UI頂点]) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(std::mem::size_of_val(頂点一覧));
    for 頂点 in 頂点一覧 {
        for 成分 in 頂点.位置px {
            バイト列.extend_from_slice(&成分.to_le_bytes());
        }
        for 成分 in 頂点.uv {
            バイト列.extend_from_slice(&成分.to_le_bytes());
        }
        バイト列.extend_from_slice(&頂点.色rgba8);
    }
    バイト列
}

pub(super) fn インデックスをバイト列にする(インデックス一覧: &[u32]) -> Vec<u8> {
    let mut バイト列 = Vec::with_capacity(インデックス一覧.len() * 4);
    for &インデックス in インデックス一覧 {
        バイト列.extend_from_slice(&インデックス.to_le_bytes());
    }
    バイト列
}
